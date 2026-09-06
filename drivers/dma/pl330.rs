//! Automatically rewritten from C to Rust
//! Source: drivers/dma/pl330.c
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
// Copyright (c) 2012 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Copyright (C) 2010 Samsung Electronics Co. Ltd.
// Jaswinder Singh <jassi.brar@samsung.com>
//

pub const PL330_MAX_CHAN: c_int = 8;
pub const PL330_MAX_IRQS: c_int = 32;
pub const PL330_MAX_PERI: c_int = 32;
pub const PL330_MAX_BURST: c_int = 16;

    enum pl330_cachectrl {
    CCTRL0,		/* Noncacheable and nonbufferable */
    CCTRL1,		/* Bufferable only */
    CCTRL2,		/* Cacheable, but do not allocate */
    CCTRL3,		/* Cacheable and bufferable, but do not allocate */
    INVALID1,	/* AWCACHE = 0x1000 */
    INVALID2,
    CCTRL6,		/* Cacheable write-through, allocate on writes only */
    CCTRL7,		/* Cacheable write-back, allocate on writes only */
    };
    enum pl330_byteswap {
    SWAP_NO,
    SWAP_2,
    SWAP_4,
    SWAP_8,
    SWAP_16,
    };
// Register and Bit field Definitions
pub const DS: c_uint = 0x0;
pub const DS_ST_STOP: c_uint = 0x0;
pub const DS_ST_EXEC: c_uint = 0x1;
pub const DS_ST_CMISS: c_uint = 0x2;
pub const DS_ST_UPDTPC: c_uint = 0x3;
pub const DS_ST_WFE: c_uint = 0x4;
pub const DS_ST_ATBRR: c_uint = 0x5;
pub const DS_ST_QBUSY: c_uint = 0x6;
pub const DS_ST_WFP: c_uint = 0x7;
pub const DS_ST_KILL: c_uint = 0x8;
pub const DS_ST_CMPLT: c_uint = 0x9;
pub const DS_ST_FLTCMP: c_uint = 0xe;
pub const DS_ST_FAULT: c_uint = 0xf;
pub const DPC: c_uint = 0x4;
pub const INTEN: c_uint = 0x20;
pub const ES: c_uint = 0x24;
pub const INTSTATUS: c_uint = 0x28;
pub const INTCLR: c_uint = 0x2c;
pub const FSM: c_uint = 0x30;
pub const FSC: c_uint = 0x34;
pub const FTM: c_uint = 0x38;
pub const _FTC: c_uint = 0x40;

pub const _CS: c_uint = 0x100;

pub const _CPC: c_uint = 0x104;

pub const _SA: c_uint = 0x400;

pub const _DA: c_uint = 0x404;

pub const _CC: c_uint = 0x408;

pub const CC_SRCBRSTLEN_SHFT: c_int = 4;
pub const CC_DSTBRSTLEN_SHFT: c_int = 18;
pub const CC_SRCBRSTSIZE_SHFT: c_int = 1;
pub const CC_DSTBRSTSIZE_SHFT: c_int = 15;
pub const CC_SRCCCTRL_SHFT: c_int = 11;
pub const CC_SRCCCTRL_MASK: c_uint = 0x7;
pub const CC_DSTCCTRL_SHFT: c_int = 25;
pub const CC_DRCCCTRL_MASK: c_uint = 0x7;
pub const CC_SWAP_SHFT: c_int = 28;
pub const _LC0: c_uint = 0x40c;

pub const _LC1: c_uint = 0x410;

pub const DBGSTATUS: c_uint = 0xd00;

pub const DBGCMD: c_uint = 0xd04;
pub const DBGINST0: c_uint = 0xd08;
pub const DBGINST1: c_uint = 0xd0c;
pub const CR0: c_uint = 0xe00;
pub const CR1: c_uint = 0xe04;
pub const CR2: c_uint = 0xe08;
pub const CR3: c_uint = 0xe0c;
pub const CR4: c_uint = 0xe10;
pub const CRD: c_uint = 0xe14;
pub const PERIPH_ID: c_uint = 0xfe0;
pub const PERIPH_REV_SHIFT: c_int = 20;
pub const PERIPH_REV_MASK: c_uint = 0xf;
pub const PERIPH_REV_R0P0: c_int = 0;
pub const PERIPH_REV_R1P0: c_int = 1;
pub const PERIPH_REV_R1P1: c_int = 2;

pub const CR0_NUM_CHANS_SHIFT: c_int = 4;
pub const CR0_NUM_CHANS_MASK: c_uint = 0x7;
pub const CR0_NUM_PERIPH_SHIFT: c_int = 12;
pub const CR0_NUM_PERIPH_MASK: c_uint = 0x1f;
pub const CR0_NUM_EVENTS_SHIFT: c_int = 17;
pub const CR0_NUM_EVENTS_MASK: c_uint = 0x1f;
pub const CR1_ICACHE_LEN_SHIFT: c_int = 0;
pub const CR1_ICACHE_LEN_MASK: c_uint = 0x7;
pub const CR1_NUM_ICACHELINES_SHIFT: c_int = 4;
pub const CR1_NUM_ICACHELINES_MASK: c_uint = 0xf;
pub const CRD_DATA_WIDTH_SHIFT: c_int = 0;
pub const CRD_DATA_WIDTH_MASK: c_uint = 0x7;
pub const CRD_WR_CAP_SHIFT: c_int = 4;
pub const CRD_WR_CAP_MASK: c_uint = 0x7;
pub const CRD_WR_Q_DEP_SHIFT: c_int = 8;
pub const CRD_WR_Q_DEP_MASK: c_uint = 0xf;
pub const CRD_RD_CAP_SHIFT: c_int = 12;
pub const CRD_RD_CAP_MASK: c_uint = 0x7;
pub const CRD_RD_Q_DEP_SHIFT: c_int = 16;
pub const CRD_RD_Q_DEP_MASK: c_uint = 0xf;
pub const CRD_DATA_BUFF_SHIFT: c_int = 20;
pub const CRD_DATA_BUFF_MASK: c_uint = 0x3ff;
pub const PART: c_uint = 0x330;
pub const DESIGNER: c_uint = 0x41;
pub const REVISION: c_uint = 0x0;
pub const INTEG_CFG: c_uint = 0x0;

    | PL330_STATE_WFE | PL330_STATE_FAULTING)
pub const CMD_DMAADDH: c_uint = 0x54;
pub const CMD_DMAEND: c_uint = 0x00;
pub const CMD_DMAFLUSHP: c_uint = 0x35;
pub const CMD_DMAGO: c_uint = 0xa0;
pub const CMD_DMALD: c_uint = 0x04;
pub const CMD_DMALDP: c_uint = 0x25;
pub const CMD_DMALP: c_uint = 0x20;
pub const CMD_DMALPEND: c_uint = 0x28;
pub const CMD_DMAKILL: c_uint = 0x01;
pub const CMD_DMAMOV: c_uint = 0xbc;
pub const CMD_DMANOP: c_uint = 0x18;
pub const CMD_DMARMB: c_uint = 0x12;
pub const CMD_DMASEV: c_uint = 0x34;
pub const CMD_DMAST: c_uint = 0x08;
pub const CMD_DMASTP: c_uint = 0x29;
pub const CMD_DMASTZ: c_uint = 0x0c;
pub const CMD_DMAWFE: c_uint = 0x36;
pub const CMD_DMAWFP: c_uint = 0x30;
pub const CMD_DMAWMB: c_uint = 0x13;
pub const SZ_DMAADDH: c_int = 3;
pub const SZ_DMAEND: c_int = 1;
pub const SZ_DMAFLUSHP: c_int = 2;
pub const SZ_DMALD: c_int = 1;
pub const SZ_DMALDP: c_int = 2;
pub const SZ_DMALP: c_int = 2;
pub const SZ_DMALPEND: c_int = 2;
pub const SZ_DMAKILL: c_int = 1;
pub const SZ_DMAMOV: c_int = 6;
pub const SZ_DMANOP: c_int = 1;
pub const SZ_DMARMB: c_int = 1;
pub const SZ_DMASEV: c_int = 2;
pub const SZ_DMAST: c_int = 1;
pub const SZ_DMASTP: c_int = 2;
pub const SZ_DMASTZ: c_int = 1;
pub const SZ_DMAWFE: c_int = 2;
pub const SZ_DMAWFP: c_int = 2;
pub const SZ_DMAWMB: c_int = 1;
pub const SZ_DMAGO: c_int = 6;

//
// With 256 bytes, we can do more than 2.5MB and 5MB xfers per req
// at 1byte/burst for P<->M and M<->M respectively.
// For typical scenario, at 1word/burst, 10MB and 20MB xfers per req
// should be enough for P<->M and M<->M respectively.
//
pub const MCODE_BUFF_PER_REQ: c_int = 256;
// Use this _only_ to wait on transient states

    static unsigned cmd_line;

    printk("%x:", cmd_line); \
    printk(KERN_CONT x); \
    cmd_line += off; \
    } while (0)

// The number of default descriptors
pub const NR_DEFAULT_DESC: c_int = 16;
// Delay for runtime PM autosuspend, ms
pub const PL330_AUTOSUSPEND_DELAY: c_int = 20;
// Populated by the PL330 core driver for DMA API driver's info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pl330_config {
    pub periph_id: u32,

    pub mode: c_uint,
    pub /: *mut *mut unsigned int data_bus_width:10; / In number of bits,
    pub data_buf_dep:11: c_uint,
    pub num_chan:4: c_uint,
    pub num_peri:6: c_uint,
    pub peri_ns: u32,
    pub num_events:6: c_uint,
    pub irq_ns: u32,
}

//
// Request Configuration.
// The PL330 core does not modify this and uses the last
// working configuration if the request doesn't provide any.
//
// The Client may want to provide this info only for the
// first request and a request with new settings.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pl330_reqcfg {
// Address Incrementing
    pub dst_inc:1: unsigned,
    pub src_inc:1: unsigned,
//
// For now, the SRC & DST protection levels
// and burst size/length are assumed same.
//
    pub nonsecure: bool,
    pub privileged: bool,
    pub insnaccess: bool,
    pub brst_len:5: unsigned,
    pub /: *mut *mut unsigned brst_size:3; / in power of 2,
    pub dcctl: enum pl330_cachectrl,
    pub scctl: enum pl330_cachectrl,
    pub swap: enum pl330_byteswap,
    pub pcfg: *mut pl330_config,
}

//
// One cycle of DMAC operation.
// There may be more than one xfer in a request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pl330_xfer {
    pub src_addr: u32,
    pub dst_addr: u32,
// Size to xfer
    pub bytes: u32,
}

// The xfer callbacks are made with one of these arguments.
    enum pl330_op_err {
// The all xfers in the request were success.
    PL330_ERR_NONE,
// If req aborted due to global error.
    PL330_ERR_ABORT,
// If req failed due to problem with Channel.
    PL330_ERR_FAIL,
    };
    enum dmamov_dst {
    SAR = 0,
    CCR,
    DAR,
    };
    enum pl330_dst {
    SRC = 0,
    DST,
    };
    enum pl330_cond {
    SINGLE,
    BURST,
    ALWAYS,
    };
    struct dma_pl330_desc;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _pl330_req {
    pub mc_bus: u32,
    pub mc_cpu: *mut c_void,
    pub desc: *mut dma_pl330_desc,
}

// ToBeDone for tasklet
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _pl330_tbd {
    pub reset_dmac: bool,
    pub reset_mngr: bool,
    pub reset_chan: u8,
}

// A DMAC Thread
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pl330_thread {
    pub id: u8,
    pub ev: c_int,
// If the channel is not yet acquired by any client
    pub free: bool,
// Parent DMAC
    pub dmac: *mut pl330_dmac,
// Only two at a time
    pub req: [_pl330_req; 2],
// Index of the last enqueued request
    pub lstenq: unsigned,
// Index of the last submitted request or -1 if the DMA is stopped
    pub req_running: c_int,
}

    enum pl330_dmac_state {
    UNINIT,
    INIT,
    DYING,
    };
    enum desc_status {
// In the DMAC pool
    FREE,
//
// Allocated to some channel during prep_xxx
// Also may be sitting on the work_list.
//
    PREP,
//
// Sitting on the work_list and already submitted
// to the PL330 core. Not more than two descriptors
// of a channel can be BUSY at any time.
//
    BUSY,
//
// Pause was called while descriptor was BUSY. Due to hardware
// limitations, only termination is possible for descriptors
// that have been paused.
//
    PAUSED,
//
// Sitting on the channel work_list but xfer done
// by PL330 core
//
    DONE,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_pl330_chan {
// Schedule desc completion
    pub task: tasklet_struct,
// DMA-Engine Channel
    pub chan: dma_chan,
// List of submitted descriptors
    pub submitted_list: list_head,
// List of issued descriptors
    pub work_list: list_head,
// List of completed descriptors
    pub completed_list: list_head,
// Pointer to the DMAC that manages this channel,
// NULL if the channel is available to be acquired.
// As the parent, this DMAC also provides descriptors
// to the channel.
//
    pub dmac: *mut pl330_dmac,
// To protect channel manipulation
    pub lock: spinlock_t,
//
// Hardware channel thread of PL330 DMAC. NULL if the channel is
// available.
//
    pub thread: *mut pl330_thread,
// For D-to-M and M-to-D channels
    pub /: *mut *mut int burst_sz; / the peripheral fifo width,
    pub /: *mut *mut int burst_len; / the number of burst,
    pub fifo_addr: phys_addr_t,
// DMA-mapped view of the FIFO; may differ if an IOMMU is present
    pub fifo_dma: dma_addr_t,
    pub dir: enum dma_data_direction,
    pub slave_config: dma_slave_config,
// for cyclic capability
    pub cyclic: bool,
// for runtime pm tracking
    pub active: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pl330_dmac {
// DMA-Engine Device
    pub ddma: dma_device,
// Pool of descriptors available for the DMAC's channels
    pub desc_pool: list_head,
// To protect desc_pool manipulation
    pub pool_lock: spinlock_t,
// Size of MicroCode buffers for each channel.
    pub mcbufsz: unsigned,
// ioremap'ed address of PL330 registers.
    pub base: *mut void __iomem,
// Populated by the PL330 core driver during pl330_add
    pub pcfg: pl330_config,
    pub lock: spinlock_t,
// Maximum possible events/irqs
    pub events: [c_int; 32],
// BUS address of MicroCode buffer
    pub mcode_bus: dma_addr_t,
// CPU address of MicroCode buffer
    pub mcode_cpu: *mut c_void,
// List of all Channel threads
    pub channels: *mut pl330_thread,
// Pointer to the MANAGER thread
    pub manager: *mut pl330_thread,
// To handle bad news in interrupt
    pub tasks: tasklet_struct,
    pub dmac_tbd: _pl330_tbd,
// State of DMAC operation
    pub state: enum pl330_dmac_state,
// Holds list of reqs with due callbacks
    pub req_done: list_head,
// Peripheral channels connected to this DMAC
    pub num_peripherals: c_uint,
    pub /: *mut *mut *mut dma_pl330_chan peripherals; / keep at end,
    pub quirks: c_int,
    pub dbgfs: *mut dentry,
    pub rstc: *mut reset_control,
    pub rstc_ocp: *mut reset_control,
}

    static struct pl330_of_quirks {
    char *quirk;
    int id;
    } of_quirks[] = {
    {
    .quirk = "arm,pl330-broken-no-flushp",
    .id = PL330_QUIRK_BROKEN_NO_FLUSHP,
    },
    {
    .quirk = "arm,pl330-periph-burst",
    .id = PL330_QUIRK_PERIPH_BURST,
    }
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_pl330_desc {
// To attach to a queue as child
    pub node: list_head,
// Descriptor for the DMA Engine API
    pub txd: dma_async_tx_descriptor,
// Xfer for PL330 core
    pub px: pl330_xfer,
    pub rqcfg: pl330_reqcfg,
    pub status: enum desc_status,
    pub bytes_requested: c_int,
    pub last: bool,
// The channel which currently holds this desc
    pub pchan: *mut dma_pl330_chan,
    pub rqtype: enum dma_transfer_direction,
// Index of peripheral for the xfer.
    pub peri:5: unsigned,
// Hook to attach to DMAC's list of reqs with due callback
    pub rqd: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _xfer_spec {
    pub ccr: u32,
    pub desc: *mut dma_pl330_desc,
}

    static int pl330_config_write(struct dma_chan *chan,
    struct dma_slave_config *slave_config,
    enum dma_transfer_direction direction);
#[no_mangle]
pub unsafe extern "C" fn _queue_full(thrd: *mut pl330_thread) -> bool {
    static inline bool _queue_full(struct pl330_thread *thrd)
    {
    return thrd.req[0].desc != core::ptr::null_mut() && thrd.req[1].desc != core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn is_manager(thrd: *mut pl330_thread) -> bool {
    static inline bool is_manager(struct pl330_thread *thrd)
    {
    return thrd.dmac.manager == thrd;
    }
// If manager of the thread is in Non-Secure mode
#[no_mangle]
pub unsafe extern "C" fn _manager_ns(thrd: *mut pl330_thread) -> bool {
    static inline bool _manager_ns(struct pl330_thread *thrd)
    {
    return (thrd.dmac.pcfg.mode & DMAC_MODE_NS) ? true : false;
    }
#[no_mangle]
pub unsafe extern "C" fn get_revision(periph_id: u32) -> u32 {
    static inline u32 get_revision(u32 periph_id)
    {
    return (periph_id >> PERIPH_REV_SHIFT) & PERIPH_REV_MASK;
    }
#[no_mangle]
pub unsafe extern "C" fn _emit_END(dry_run: unsigned, buf[]: u8) -> u32 {
    static inline u32 _emit_END(unsigned dry_run, u8 buf[])
    {
    if (dry_run)
    return SZ_DMAEND;
    buf[0] = CMD_DMAEND;
    PL330_DBGCMD_DUMP(SZ_DMAEND, "\tDMAEND\n");
    return SZ_DMAEND;
    }
#[no_mangle]
pub unsafe extern "C" fn _emit_FLUSHP(dry_run: unsigned, buf[]: u8, peri: u8) -> u32 {
    static inline u32 _emit_FLUSHP(unsigned dry_run, u8 buf[], u8 peri)
    {
    if (dry_run)
    return SZ_DMAFLUSHP;
    buf[0] = CMD_DMAFLUSHP;
    peri &= 0x1f;
    peri <<= 3;
    buf[1] = peri;
    PL330_DBGCMD_DUMP(SZ_DMAFLUSHP, "\tDMAFLUSHP %u\n", peri >> 3);
    return SZ_DMAFLUSHP;
    }
#[no_mangle]
pub unsafe extern "C" fn _emit_LD(dry_run: unsigned, buf[]: u8, cond: enum pl330_cond) -> u32 {
    static inline u32 _emit_LD(unsigned dry_run, u8 buf[],	enum pl330_cond cond)
    {
    if (dry_run)
    return SZ_DMALD;
    buf[0] = CMD_DMALD;
    if (cond == SINGLE)
    buf[0] |= (0 << 1) | (1 << 0);
#[no_mangle]
pub unsafe extern "C" fn if(BURST: cond ==) -> else {
    else if (cond == BURST)
    buf[0] |= (1 << 1) | (1 << 0);
    PL330_DBGCMD_DUMP(SZ_DMALD, "\tDMALD%c\n",
    cond == SINGLE ? 'S' : (cond == BURST ? 'B' : 'A'));
    return SZ_DMALD;
    }
    static inline u32 _emit_LDP(unsigned dry_run, u8 buf[],
    enum pl330_cond cond, u8 peri)
    {
    if (dry_run)
    return SZ_DMALDP;
    buf[0] = CMD_DMALDP;
    if (cond == BURST)
    buf[0] |= (1 << 1);
    peri &= 0x1f;
    peri <<= 3;
    buf[1] = peri;
    PL330_DBGCMD_DUMP(SZ_DMALDP, "\tDMALDP%c %u\n",
    cond == SINGLE ? 'S' : 'B', peri >> 3);
    return SZ_DMALDP;
    }
    static inline u32 _emit_LP(unsigned dry_run, u8 buf[],
    unsigned loop, u8 cnt)
    {
    if (dry_run)
    return SZ_DMALP;
    buf[0] = CMD_DMALP;
    if (loop)
    buf[0] |= (1 << 1);
    cnt--; /* DMAC increments by 1 internally */
    buf[1] = cnt;
    PL330_DBGCMD_DUMP(SZ_DMALP, "\tDMALP_%c %u\n", loop ? '1' : '0', cnt);
    return SZ_DMALP;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _arg_LPEND {
    pub cond: enum pl330_cond,
    pub forever: bool,
    pub loop: unsigned,
    pub bjump: u8,
}

    static inline u32 _emit_LPEND(unsigned dry_run, u8 buf[],
    const struct _arg_LPEND *arg)
    {
    let mut cond: enum pl330_cond = arg.cond;
    let mut forever: bool = arg.forever;
    let mut loop: unsigned = arg.loop;
    let mut bjump: u8 = arg.bjump;
    if (dry_run)
    return SZ_DMALPEND;
    buf[0] = CMD_DMALPEND;
    if (loop)
    buf[0] |= (1 << 2);
    if (!forever)
    buf[0] |= (1 << 4);
    if (cond == SINGLE)
    buf[0] |= (0 << 1) | (1 << 0);
#[no_mangle]
pub unsafe extern "C" fn if(BURST: cond ==) -> else {
    else if (cond == BURST)
    buf[0] |= (1 << 1) | (1 << 0);
    buf[1] = bjump;
    PL330_DBGCMD_DUMP(SZ_DMALPEND, "\tDMALP%s%c_%c bjmpto_%x\n",
    forever ? "FE" : "END",
    cond == SINGLE ? 'S' : (cond == BURST ? 'B' : 'A'),
    loop ? '1' : '0',
    bjump);
    return SZ_DMALPEND;
    }
#[no_mangle]
pub unsafe extern "C" fn _emit_KILL(dry_run: unsigned, buf[]: u8) -> u32 {
    static inline u32 _emit_KILL(unsigned dry_run, u8 buf[])
    {
    if (dry_run)
    return SZ_DMAKILL;
    buf[0] = CMD_DMAKILL;
    return SZ_DMAKILL;
    }
    static inline u32 _emit_MOV(unsigned dry_run, u8 buf[],
    enum dmamov_dst dst, u32 val)
    {
    if (dry_run)
    return SZ_DMAMOV;
    buf[0] = CMD_DMAMOV;
    buf[1] = dst;
    buf[2] = val;
    buf[3] = val >> 8;
    buf[4] = val >> 16;
    buf[5] = val >> 24;
    PL330_DBGCMD_DUMP(SZ_DMAMOV, "\tDMAMOV %s 0x%x\n",
    dst == SAR ? "SAR" : (dst == DAR ? "DAR" : "CCR"), val);
    return SZ_DMAMOV;
    }
#[no_mangle]
pub unsafe extern "C" fn _emit_RMB(dry_run: unsigned, buf[]: u8) -> u32 {
    static inline u32 _emit_RMB(unsigned dry_run, u8 buf[])
    {
    if (dry_run)
    return SZ_DMARMB;
    buf[0] = CMD_DMARMB;
    PL330_DBGCMD_DUMP(SZ_DMARMB, "\tDMARMB\n");
    return SZ_DMARMB;
    }
#[no_mangle]
pub unsafe extern "C" fn _emit_SEV(dry_run: unsigned, buf[]: u8, ev: u8) -> u32 {
    static inline u32 _emit_SEV(unsigned dry_run, u8 buf[], u8 ev)
    {
    if (dry_run)
    return SZ_DMASEV;
    buf[0] = CMD_DMASEV;
    ev &= 0x1f;
    ev <<= 3;
    buf[1] = ev;
    PL330_DBGCMD_DUMP(SZ_DMASEV, "\tDMASEV %u\n", ev >> 3);
    return SZ_DMASEV;
    }
#[no_mangle]
pub unsafe extern "C" fn _emit_ST(dry_run: unsigned, buf[]: u8, cond: enum pl330_cond) -> u32 {
    static inline u32 _emit_ST(unsigned dry_run, u8 buf[], enum pl330_cond cond)
    {
    if (dry_run)
    return SZ_DMAST;
    buf[0] = CMD_DMAST;
    if (cond == SINGLE)
    buf[0] |= (0 << 1) | (1 << 0);
#[no_mangle]
pub unsafe extern "C" fn if(BURST: cond ==) -> else {
    else if (cond == BURST)
    buf[0] |= (1 << 1) | (1 << 0);
    PL330_DBGCMD_DUMP(SZ_DMAST, "\tDMAST%c\n",
    cond == SINGLE ? 'S' : (cond == BURST ? 'B' : 'A'));
    return SZ_DMAST;
    }
    static inline u32 _emit_STP(unsigned dry_run, u8 buf[],
    enum pl330_cond cond, u8 peri)
    {
    if (dry_run)
    return SZ_DMASTP;
    buf[0] = CMD_DMASTP;
    if (cond == BURST)
    buf[0] |= (1 << 1);
    peri &= 0x1f;
    peri <<= 3;
    buf[1] = peri;
    PL330_DBGCMD_DUMP(SZ_DMASTP, "\tDMASTP%c %u\n",
    cond == SINGLE ? 'S' : 'B', peri >> 3);
    return SZ_DMASTP;
    }
    static inline u32 _emit_WFP(unsigned dry_run, u8 buf[],
    enum pl330_cond cond, u8 peri)
    {
    if (dry_run)
    return SZ_DMAWFP;
    buf[0] = CMD_DMAWFP;
    if (cond == SINGLE)
    buf[0] |= (0 << 1) | (0 << 0);
#[no_mangle]
pub unsafe extern "C" fn if(BURST: cond ==) -> else {
    else if (cond == BURST)
    buf[0] |= (1 << 1) | (0 << 0);
    else
    buf[0] |= (0 << 1) | (1 << 0);
    peri &= 0x1f;
    peri <<= 3;
    buf[1] = peri;
    PL330_DBGCMD_DUMP(SZ_DMAWFP, "\tDMAWFP%c %u\n",
    cond == SINGLE ? 'S' : (cond == BURST ? 'B' : 'P'), peri >> 3);
    return SZ_DMAWFP;
    }
#[no_mangle]
pub unsafe extern "C" fn _emit_WMB(dry_run: unsigned, buf[]: u8) -> u32 {
    static inline u32 _emit_WMB(unsigned dry_run, u8 buf[])
    {
    if (dry_run)
    return SZ_DMAWMB;
    buf[0] = CMD_DMAWMB;
    PL330_DBGCMD_DUMP(SZ_DMAWMB, "\tDMAWMB\n");
    return SZ_DMAWMB;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _arg_GO {
    pub chan: u8,
    pub addr: u32,
    pub ns: unsigned,
}

    static inline u32 _emit_GO(unsigned dry_run, u8 buf[],
    const struct _arg_GO *arg)
    {
    let mut chan: u8 = arg.chan;
    let mut addr: u32 = arg.addr;
    let mut ns: unsigned = arg.ns;
    if (dry_run)
    return SZ_DMAGO;
    buf[0] = CMD_DMAGO;
    buf[0] |= (ns << 1);
    buf[1] = chan & 0x7;
    buf[2] = addr;
    buf[3] = addr >> 8;
    buf[4] = addr >> 16;
    buf[5] = addr >> 24;
    return SZ_DMAGO;
    }

// Returns Time-Out
#[no_mangle]
unsafe extern "C" fn _until_dmac_idle(thrd: *mut pl330_thread) -> bool {
    static bool _until_dmac_idle(struct pl330_thread *thrd)
    {
    void __iomem *regs = thrd.dmac.base;
    let mut loops: c_ulong = msecs_to_loops(5);
    do {
// Until Manager is Idle
    if (!(readl(regs + DBGSTATUS) & DBG_BUSY))
    break;
    cpu_relax();
    } while (--loops);
    if (!loops)
    return true;
    return false;
    }
    static inline void _execute_DBGINSN(struct pl330_thread *thrd,
    u8 insn[], bool as_manager)
    {
    void __iomem *regs = thrd.dmac.base;
    u32 val;
// If timed out due to halted state-machine
    if (_until_dmac_idle(thrd)) {
    dev_err(thrd.dmac.ddma.dev, "DMAC halted!\n");
    return;
    }
    val = (insn[0] << 16) | (insn[1] << 24);
    if (!as_manager) {
    val |= (1 << 0);
    val |= (thrd.id << 8); /* Channel Number */
    }
    writel(val, regs + DBGINST0);
    val = le32_to_cpu(*((__le32 *)&insn[2]));
    writel(val, regs + DBGINST1);
// Get going
    writel(0, regs + DBGCMD);
    }
#[no_mangle]
pub unsafe extern "C" fn _state(thrd: *mut pl330_thread) -> u32 {
    static inline u32 _state(struct pl330_thread *thrd)
    {
    void __iomem *regs = thrd.dmac.base;
    u32 val;
    if (is_manager(thrd))
    val = readl(regs + DS) & 0xf;
    else
    val = readl(regs + CS(thrd.id)) & 0xf;
    switch (val) {
    case DS_ST_STOP:
    return PL330_STATE_STOPPED;
    case DS_ST_EXEC:
    return PL330_STATE_EXECUTING;
    case DS_ST_CMISS:
    return PL330_STATE_CACHEMISS;
    case DS_ST_UPDTPC:
    return PL330_STATE_UPDTPC;
    case DS_ST_WFE:
    return PL330_STATE_WFE;
    case DS_ST_FAULT:
    return PL330_STATE_FAULTING;
    case DS_ST_ATBRR:
    if (is_manager(thrd))
    return PL330_STATE_INVALID;
    else
    return PL330_STATE_ATBARRIER;
    case DS_ST_QBUSY:
    if (is_manager(thrd))
    return PL330_STATE_INVALID;
    else
    return PL330_STATE_QUEUEBUSY;
    case DS_ST_WFP:
    if (is_manager(thrd))
    return PL330_STATE_INVALID;
    else
    return PL330_STATE_WFP;
    case DS_ST_KILL:
    if (is_manager(thrd))
    return PL330_STATE_INVALID;
    else
    return PL330_STATE_KILLING;
    case DS_ST_CMPLT:
    if (is_manager(thrd))
    return PL330_STATE_INVALID;
    else
    return PL330_STATE_COMPLETING;
    case DS_ST_FLTCMP:
    if (is_manager(thrd))
    return PL330_STATE_INVALID;
    else
    return PL330_STATE_FAULT_COMPLETING;
    default:
    return PL330_STATE_INVALID;
    }
    }
#[no_mangle]
unsafe extern "C" fn _stop(thrd: *mut pl330_thread) {
    static void _stop(struct pl330_thread *thrd)
    {
    void __iomem *regs = thrd.dmac.base;
    u8 insn[6] = {0, 0, 0, 0, 0, 0};
    let mut inten: u32 = readl(regs + INTEN);
    if (_state(thrd) == PL330_STATE_FAULT_COMPLETING)
    UNTIL(thrd, PL330_STATE_FAULTING | PL330_STATE_KILLING);
// Return if nothing needs to be done
    if (_state(thrd) == PL330_STATE_COMPLETING
    || _state(thrd) == PL330_STATE_KILLING
    || _state(thrd) == PL330_STATE_STOPPED)
    return;
    _emit_KILL(0, insn);
    _execute_DBGINSN(thrd, insn, is_manager(thrd));
// clear the event
    if (inten & (1 << thrd.ev))
    writel(1 << thrd.ev, regs + INTCLR);
// Stop generating interrupts for SEV
    writel(inten & ~(1 << thrd.ev), regs + INTEN);
    }
// Start doing req 'idx' of thread 'thrd'
#[no_mangle]
unsafe extern "C" fn _trigger(thrd: *mut pl330_thread) -> bool {
    static bool _trigger(struct pl330_thread *thrd)
    {
    void __iomem *regs = thrd.dmac.base;
    struct _pl330_req *req;
    struct dma_pl330_desc *desc;
    struct _arg_GO go;
    unsigned ns;
    u8 insn[6] = {0, 0, 0, 0, 0, 0};
    int idx;
// Return if already ACTIVE
    if (_state(thrd) != PL330_STATE_STOPPED)
    return true;
    idx = 1 - thrd.lstenq;
    if (thrd.req[idx].desc != core::ptr::null_mut()) {
    req = &thrd.req[idx];
    } else {
    idx = thrd.lstenq;
    if (thrd.req[idx].desc != core::ptr::null_mut())
    req = &thrd.req[idx];
    else
    req = core::ptr::null_mut();
    }
// Return if no request
    if (!req)
    return true;
// Return if req is running
    if (idx == thrd.req_running)
    return true;
    desc = req.desc;
    ns = desc.rqcfg.nonsecure ? 1 : 0;
// See 'Abort Sources' point-4 at Page 2-25
    if (_manager_ns(thrd) && !ns)
    dev_info(thrd.dmac.ddma.dev, "%s:%d Recipe for ABORT!\n",
    __func__, __LINE__);
    go.chan = thrd.id;
    go.addr = req.mc_bus;
    go.ns = ns;
    _emit_GO(0, insn, &go);
// Set to generate interrupts for SEV
    writel(readl(regs + INTEN) | (1 << thrd.ev), regs + INTEN);
// Only manager can execute GO
    _execute_DBGINSN(thrd, insn, true);
    thrd.req_running = idx;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn pl330_start_thread(thrd: *mut pl330_thread) -> bool {
    static bool pl330_start_thread(struct pl330_thread *thrd)
    {
    switch (_state(thrd)) {
    case PL330_STATE_FAULT_COMPLETING:
    UNTIL(thrd, PL330_STATE_FAULTING | PL330_STATE_KILLING);
    if (_state(thrd) == PL330_STATE_KILLING)
    UNTIL(thrd, PL330_STATE_STOPPED)
    fallthrough;
    case PL330_STATE_FAULTING:
    _stop(thrd);
    fallthrough;
    case PL330_STATE_KILLING:
    case PL330_STATE_COMPLETING:
    UNTIL(thrd, PL330_STATE_STOPPED)
    fallthrough;
    case PL330_STATE_STOPPED:
    return _trigger(thrd);
    case PL330_STATE_WFP:
    case PL330_STATE_QUEUEBUSY:
    case PL330_STATE_ATBARRIER:
    case PL330_STATE_UPDTPC:
    case PL330_STATE_CACHEMISS:
    case PL330_STATE_EXECUTING:
    return true;
    case PL330_STATE_WFE: /* For RESUME, nothing yet */
    default:
    return false;
    }
    }
    static inline int _ldst_memtomem(unsigned dry_run, u8 buf[],
    const struct _xfer_spec *pxs, int cyc)
    {
    let mut off: c_int = 0;
    struct pl330_config *pcfg = pxs.desc.rqcfg.pcfg;
// check lock-up free version
    if (get_revision(pcfg.periph_id) >= PERIPH_REV_R1P0) {
    while (cyc--) {
    off += _emit_LD(dry_run, &buf[off], ALWAYS);
    off += _emit_ST(dry_run, &buf[off], ALWAYS);
    }
    } else {
    while (cyc--) {
    off += _emit_LD(dry_run, &buf[off], ALWAYS);
    off += _emit_RMB(dry_run, &buf[off]);
    off += _emit_ST(dry_run, &buf[off], ALWAYS);
    off += _emit_WMB(dry_run, &buf[off]);
    }
    }
    return off;
    }
    static u32 _emit_load(unsigned int dry_run, u8 buf[],
    enum pl330_cond cond, enum dma_transfer_direction direction,
    u8 peri)
    {
    let mut off: c_int = 0;
    switch (direction) {
    case DMA_MEM_TO_MEM:
    case DMA_MEM_TO_DEV:
    off += _emit_LD(dry_run, &buf[off], cond);
    break;
    case DMA_DEV_TO_MEM:
    if (cond == ALWAYS) {
    off += _emit_LDP(dry_run, &buf[off], SINGLE,
    peri);
    off += _emit_LDP(dry_run, &buf[off], BURST,
    peri);
    } else {
    off += _emit_LDP(dry_run, &buf[off], cond,
    peri);
    }
    break;
    default:
// this code should be unreachable
    WARN_ON(1);
    break;
    }
    return off;
    }
    static inline u32 _emit_store(unsigned int dry_run, u8 buf[],
    enum pl330_cond cond, enum dma_transfer_direction direction,
    u8 peri)
    {
    let mut off: c_int = 0;
    switch (direction) {
    case DMA_MEM_TO_MEM:
    case DMA_DEV_TO_MEM:
    off += _emit_ST(dry_run, &buf[off], cond);
    break;
    case DMA_MEM_TO_DEV:
    if (cond == ALWAYS) {
    off += _emit_STP(dry_run, &buf[off], SINGLE,
    peri);
    off += _emit_STP(dry_run, &buf[off], BURST,
    peri);
    } else {
    off += _emit_STP(dry_run, &buf[off], cond,
    peri);
    }
    break;
    default:
// this code should be unreachable
    WARN_ON(1);
    break;
    }
    return off;
    }
    static inline int _ldst_peripheral(struct pl330_dmac *pl330,
    unsigned dry_run, u8 buf[],
    const struct _xfer_spec *pxs, int cyc,
    enum pl330_cond cond)
    {
    let mut off: c_int = 0;
//
// do FLUSHP at beginning to clear any stale dma requests before the
// first WFP.
//
    if (!(pl330.quirks & PL330_QUIRK_BROKEN_NO_FLUSHP))
    off += _emit_FLUSHP(dry_run, &buf[off], pxs.desc.peri);
    while (cyc--) {
    off += _emit_WFP(dry_run, &buf[off], cond, pxs.desc.peri);
    off += _emit_load(dry_run, &buf[off], cond, pxs.desc.rqtype,
    pxs.desc.peri);
    off += _emit_store(dry_run, &buf[off], cond, pxs.desc.rqtype,
    pxs.desc.peri);
    }
    return off;
    }
    static int _bursts(struct pl330_dmac *pl330, unsigned dry_run, u8 buf[],
    const struct _xfer_spec *pxs, int cyc)
    {
    let mut off: c_int = 0;
    let mut cond: enum pl330_cond = BRST_LEN(pxs.ccr) > 1 ? BURST : SINGLE;
    if (pl330.quirks & PL330_QUIRK_PERIPH_BURST)
    cond = BURST;
    switch (pxs.desc.rqtype) {
    case DMA_MEM_TO_DEV:
    case DMA_DEV_TO_MEM:
    off += _ldst_peripheral(pl330, dry_run, &buf[off], pxs, cyc,
    cond);
    break;
    case DMA_MEM_TO_MEM:
    off += _ldst_memtomem(dry_run, &buf[off], pxs, cyc);
    break;
    default:
// this code should be unreachable
    WARN_ON(1);
    break;
    }
    return off;
    }
//
// only the unaligned burst transfers have the dregs.
// so, still transfer dregs with a reduced size burst
// for mem-to-mem, mem-to-dev or dev-to-mem.
//
    static int _dregs(struct pl330_dmac *pl330, unsigned int dry_run, u8 buf[],
    const struct _xfer_spec *pxs, int transfer_length)
    {
    let mut off: c_int = 0;
    int dregs_ccr;
    if (transfer_length == 0)
    return off;
//
// dregs_len = (total bytes - BURST_TO_BYTE(bursts, ccr))
// BRST_SIZE(ccr)
// the dregs len must be smaller than burst len,
// so, for higher efficiency, we can modify CCR
// to use a reduced size burst len for the dregs.
//
    dregs_ccr = pxs.ccr;
    dregs_ccr &= ~((0xf << CC_SRCBRSTLEN_SHFT) |
    (0xf << CC_DSTBRSTLEN_SHFT));
    dregs_ccr |= (((transfer_length - 1) & 0xf) <<
    CC_SRCBRSTLEN_SHFT);
    dregs_ccr |= (((transfer_length - 1) & 0xf) <<
    CC_DSTBRSTLEN_SHFT);
    switch (pxs.desc.rqtype) {
    case DMA_MEM_TO_DEV:
    case DMA_DEV_TO_MEM:
    off += _emit_MOV(dry_run, &buf[off], CCR, dregs_ccr);
    off += _ldst_peripheral(pl330, dry_run, &buf[off], pxs, 1,
    BURST);
    break;
    case DMA_MEM_TO_MEM:
    off += _emit_MOV(dry_run, &buf[off], CCR, dregs_ccr);
    off += _ldst_memtomem(dry_run, &buf[off], pxs, 1);
    break;
    default:
// this code should be unreachable
    WARN_ON(1);
    break;
    }
    return off;
    }
// Returns bytes consumed and updates bursts
    static inline int _loop(struct pl330_dmac *pl330, unsigned dry_run, u8 buf[],
    unsigned long *bursts, const struct _xfer_spec *pxs)
    {
    int cyc, cycmax, szlp, szlpend, szbrst, off;
    unsigned lcnt0, lcnt1, ljmp0, ljmp1;
    struct _arg_LPEND lpend;
    if (*bursts == 1)
    return _bursts(pl330, dry_run, buf, pxs, 1);
// Max iterations possible in DMALP is 256
    if (*bursts >= 256*256) {
    lcnt1 = 256;
    lcnt0 = 256;
    cyc = *bursts / lcnt1 / lcnt0;
    } else if (*bursts > 256) {
    lcnt1 = 256;
    lcnt0 = *bursts / lcnt1;
    cyc = 1;
    } else {
    lcnt1 = *bursts;
    lcnt0 = 0;
    cyc = 1;
    }
    szlp = _emit_LP(1, buf, 0, 0);
    szbrst = _bursts(pl330, 1, buf, pxs, 1);
    lpend.cond = ALWAYS;
    lpend.forever = false;
    lpend.loop = 0;
    lpend.bjump = 0;
    szlpend = _emit_LPEND(1, buf, &lpend);
    if (lcnt0) {
    szlp *= 2;
    szlpend *= 2;
    }
//
// Max bursts that we can unroll due to limit on the
// size of backward jump that can be encoded in DMALPEND
// which is 8-bits and hence 255
//
    cycmax = (255 - (szlp + szlpend)) / szbrst;
    cyc = (cycmax < cyc) ? cycmax : cyc;
    off = 0;
    if (lcnt0) {
    off += _emit_LP(dry_run, &buf[off], 0, lcnt0);
    ljmp0 = off;
    }
    off += _emit_LP(dry_run, &buf[off], 1, lcnt1);
    ljmp1 = off;
    off += _bursts(pl330, dry_run, &buf[off], pxs, cyc);
    lpend.cond = ALWAYS;
    lpend.forever = false;
    lpend.loop = 1;
    lpend.bjump = off - ljmp1;
    off += _emit_LPEND(dry_run, &buf[off], &lpend);
    if (lcnt0) {
    lpend.cond = ALWAYS;
    lpend.forever = false;
    lpend.loop = 0;
    lpend.bjump = off - ljmp0;
    off += _emit_LPEND(dry_run, &buf[off], &lpend);
    }
// bursts = lcnt1 * cyc;
    if (lcnt0)
// bursts *= lcnt0;
    return off;
    }
    static inline int _setup_loops(struct pl330_dmac *pl330,
    unsigned dry_run, u8 buf[],
    const struct _xfer_spec *pxs)
    {
    struct pl330_xfer *x = &pxs.desc.px;
    let mut ccr: u32 = pxs.ccr;
    unsigned long c, bursts = BYTE_TO_BURST(x.bytes, ccr);
    int num_dregs = (x.bytes - BURST_TO_BYTE(bursts, ccr)) /
    BRST_SIZE(ccr);
    let mut off: c_int = 0;
    while (bursts) {
    c = bursts;
    off += _loop(pl330, dry_run, &buf[off], &c, pxs);
    bursts -= c;
    }
    off += _dregs(pl330, dry_run, &buf[off], pxs, num_dregs);
    return off;
    }
    static inline int _setup_xfer(struct pl330_dmac *pl330,
    unsigned dry_run, u8 buf[],
    const struct _xfer_spec *pxs)
    {
    struct pl330_xfer *x = &pxs.desc.px;
    let mut off: c_int = 0;
// DMAMOV SAR, x->src_addr
    off += _emit_MOV(dry_run, &buf[off], SAR, x.src_addr);
// DMAMOV DAR, x->dst_addr
    off += _emit_MOV(dry_run, &buf[off], DAR, x.dst_addr);
// Setup Loop(s)
    off += _setup_loops(pl330, dry_run, &buf[off], pxs);
    return off;
    }
//
// A req is a sequence of one or more xfer units.
// Returns the number of bytes taken to setup the MC for the req.
//
    static int _setup_req(struct pl330_dmac *pl330, unsigned dry_run,
    struct pl330_thread *thrd, unsigned index,
    struct _xfer_spec *pxs)
    {
    struct _pl330_req *req = &thrd.req[index];
    u8 *buf = req.mc_cpu;
    let mut off: c_int = 0;
    PL330_DBGMC_START(req.mc_bus);
// DMAMOV CCR, ccr
    off += _emit_MOV(dry_run, &buf[off], CCR, pxs.ccr);
    off += _setup_xfer(pl330, dry_run, &buf[off], pxs);
// DMASEV peripheral/event
    off += _emit_SEV(dry_run, &buf[off], thrd.ev);
// DMAEND
    off += _emit_END(dry_run, &buf[off]);
    return off;
    }
#[no_mangle]
pub unsafe extern "C" fn _prepare_ccr(rqc: *const pl330_reqcfg) -> u32 {
    static inline u32 _prepare_ccr(const struct pl330_reqcfg *rqc)
    {
    let mut ccr: u32 = 0;
    if (rqc.src_inc)
    ccr |= CC_SRCINC;
    if (rqc.dst_inc)
    ccr |= CC_DSTINC;
// We set same protection levels for Src and DST for now
    if (rqc.privileged)
    ccr |= CC_SRCPRI | CC_DSTPRI;
    if (rqc.nonsecure)
    ccr |= CC_SRCNS | CC_DSTNS;
    if (rqc.insnaccess)
    ccr |= CC_SRCIA | CC_DSTIA;
    ccr |= (((rqc.brst_len - 1) & 0xf) << CC_SRCBRSTLEN_SHFT);
    ccr |= (((rqc.brst_len - 1) & 0xf) << CC_DSTBRSTLEN_SHFT);
    ccr |= (rqc.brst_size << CC_SRCBRSTSIZE_SHFT);
    ccr |= (rqc.brst_size << CC_DSTBRSTSIZE_SHFT);
    ccr |= (rqc.scctl << CC_SRCCCTRL_SHFT);
    ccr |= (rqc.dcctl << CC_DSTCCTRL_SHFT);
    ccr |= (rqc.swap << CC_SWAP_SHFT);
    return ccr;
    }
//
// Submit a list of xfers after which the client wants notification.
// Client is not notified after each xfer unit, just once after all
// xfer units are done or some error occurs.
//
    static int pl330_submit_req(struct pl330_thread *thrd,
    struct dma_pl330_desc *desc)
    {
    struct pl330_dmac *pl330 = thrd.dmac;
    struct _xfer_spec xs;
    unsigned long flags;
    unsigned idx;
    u32 ccr;
    let mut ret: c_int = 0;
    switch (desc.rqtype) {
    case DMA_MEM_TO_DEV:
    break;
    case DMA_DEV_TO_MEM:
    break;
    case DMA_MEM_TO_MEM:
    break;
    default:
    return -ENOTSUPP;
    }
    if (pl330.state == DYING
    || pl330.dmac_tbd.reset_chan & (1 << thrd.id)) {
    dev_info(thrd.dmac.ddma.dev, "%s:%d\n",
    __func__, __LINE__);
    return -EAGAIN;
    }
// If request for non-existing peripheral
    if (desc.rqtype != DMA_MEM_TO_MEM &&
    desc.peri >= pl330.pcfg.num_peri) {
    dev_info(thrd.dmac.ddma.dev,
    "%s:%d Invalid peripheral(%u)!\n",
    __func__, __LINE__, desc.peri);
    return -EINVAL;
    }
    spin_lock_irqsave(&pl330.lock, flags);
    if (_queue_full(thrd)) {
    ret = -EAGAIN;
    goto xfer_exit;
    }
// Prefer Secure Channel
    if (!_manager_ns(thrd))
    desc.rqcfg.nonsecure = 0;
    else
    desc.rqcfg.nonsecure = 1;
    ccr = _prepare_ccr(&desc.rqcfg);
    idx = thrd.req[0].desc == core::ptr::null_mut() ? 0 : 1;
    xs.ccr = ccr;
    xs.desc = desc;
// First dry run to check if req is acceptable
    ret = _setup_req(pl330, 1, thrd, idx, &xs);
    if (ret > pl330.mcbufsz / 2) {
    dev_info(pl330.ddma.dev, "%s:%d Try increasing mcbufsz (%i/%i)\n",
    __func__, __LINE__, ret, pl330.mcbufsz / 2);
    ret = -ENOMEM;
    goto xfer_exit;
    }
// Hook the request
    thrd.lstenq = idx;
    thrd.req[idx].desc = desc;
    _setup_req(pl330, 0, thrd, idx, &xs);
    ret = 0;
    xfer_exit:
    spin_unlock_irqrestore(&pl330.lock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dma_pl330_rqcb(desc: *mut dma_pl330_desc, err: enum pl330_op_err) {
    static void dma_pl330_rqcb(struct dma_pl330_desc *desc, enum pl330_op_err err)
    {
    struct dma_pl330_chan *pch;
    unsigned long flags;
    if (!desc)
    return;
    pch = desc.pchan;
// If desc aborted
    if (!pch)
    return;
    spin_lock_irqsave(&pch.lock, flags);
    desc.status = DONE;
    spin_unlock_irqrestore(&pch.lock, flags);
    tasklet_schedule(&pch.task);
    }
#[no_mangle]
unsafe extern "C" fn pl330_dotask(t: *mut tasklet_struct) {
    static void pl330_dotask(struct tasklet_struct *t)
    {
    struct pl330_dmac *pl330 = from_tasklet(pl330, t, tasks);
    unsigned long flags;
    int i;
    spin_lock_irqsave(&pl330.lock, flags);
// The DMAC itself gone nuts
    if (pl330.dmac_tbd.reset_dmac) {
    pl330.state = DYING;
// Reset the manager too
    pl330.dmac_tbd.reset_mngr = true;
// Clear the reset flag
    pl330.dmac_tbd.reset_dmac = false;
    }
    if (pl330.dmac_tbd.reset_mngr) {
    _stop(pl330.manager);
// Reset all channels
    pl330.dmac_tbd.reset_chan = (1 << pl330.pcfg.num_chan) - 1;
// Clear the reset flag
    pl330.dmac_tbd.reset_mngr = false;
    }
    for (i = 0; i < pl330.pcfg.num_chan; i++) {
    if (pl330.dmac_tbd.reset_chan & (1 << i)) {
    struct pl330_thread *thrd = &pl330.channels[i];
    void __iomem *regs = pl330.base;
    enum pl330_op_err err;
    _stop(thrd);
    if (readl(regs + FSC) & (1 << thrd.id))
    err = PL330_ERR_FAIL;
    else
    err = PL330_ERR_ABORT;
    spin_unlock_irqrestore(&pl330.lock, flags);
    dma_pl330_rqcb(thrd.req[1 - thrd.lstenq].desc, err);
    dma_pl330_rqcb(thrd.req[thrd.lstenq].desc, err);
    spin_lock_irqsave(&pl330.lock, flags);
    thrd.req[0].desc = core::ptr::null_mut();
    thrd.req[1].desc = core::ptr::null_mut();
    thrd.req_running = -1;
// Clear the reset flag
    pl330.dmac_tbd.reset_chan &= ~(1 << i);
    }
    }
    spin_unlock_irqrestore(&pl330.lock, flags);
    return;
    }
// Returns 1 if state was updated, 0 otherwise
#[no_mangle]
unsafe extern "C" fn pl330_update(pl330: *mut pl330_dmac) -> c_int {
    static int pl330_update(struct pl330_dmac *pl330)
    {
    struct dma_pl330_desc *descdone;
    unsigned long flags;
    void __iomem *regs;
    u32 val;
    int id, ev, ret = 0;
    regs = pl330.base;
    spin_lock_irqsave(&pl330.lock, flags);
    val = readl(regs + FSM) & 0x1;
    if (val)
    pl330.dmac_tbd.reset_mngr = true;
    else
    pl330.dmac_tbd.reset_mngr = false;
    val = readl(regs + FSC) & ((1 << pl330.pcfg.num_chan) - 1);
    pl330.dmac_tbd.reset_chan |= val;
    if (val) {
    let mut i: c_int = 0;
    while (i < pl330.pcfg.num_chan) {
    if (val & (1 << i)) {
    dev_info(pl330.ddma.dev,
    "Reset Channel-%d\t CS-%x FTC-%x\n",
    i, readl(regs + CS(i)),
    readl(regs + FTC(i)));
    _stop(&pl330.channels[i]);
    }
    i++;
    }
    }
// Check which event happened i.e, thread notified
    val = readl(regs + ES);
    if (pl330.pcfg.num_events < 32
    && val & ~((1 << pl330.pcfg.num_events) - 1)) {
    pl330.dmac_tbd.reset_dmac = true;
    dev_err(pl330.ddma.dev, "%s:%d Unexpected!\n", __func__,
    __LINE__);
    ret = 1;
    goto updt_exit;
    }
    for (ev = 0; ev < pl330.pcfg.num_events; ev++) {
    if (val & (1 << ev)) { /* Event occurred */
    struct pl330_thread *thrd;
    let mut inten: u32 = readl(regs + INTEN);
    int active;
// Clear the event
    if (inten & (1 << ev))
    writel(1 << ev, regs + INTCLR);
    ret = 1;
    id = pl330.events[ev];
    thrd = &pl330.channels[id];
    active = thrd.req_running;
    if (active == -1) /* Aborted */
    continue;
// Detach the req
    descdone = thrd.req[active].desc;
    thrd.req[active].desc = core::ptr::null_mut();
    thrd.req_running = -1;
// Get going again ASAP
    pl330_start_thread(thrd);
// For now, just make a list of callbacks to be done
    list_add_tail(&descdone.rqd, &pl330.req_done);
    }
    }
// Now that we are in no hurry, do the callbacks
    while (!list_empty(&pl330.req_done)) {
    descdone = list_first_entry(&pl330.req_done,
    struct dma_pl330_desc, rqd);
    list_del(&descdone.rqd);
    spin_unlock_irqrestore(&pl330.lock, flags);
    dma_pl330_rqcb(descdone, PL330_ERR_NONE);
    spin_lock_irqsave(&pl330.lock, flags);
    }
    updt_exit:
    spin_unlock_irqrestore(&pl330.lock, flags);
    if (pl330.dmac_tbd.reset_dmac
    || pl330.dmac_tbd.reset_mngr
    || pl330.dmac_tbd.reset_chan) {
    ret = 1;
    tasklet_schedule(&pl330.tasks);
    }
    return ret;
    }
// Reserve an event
#[no_mangle]
pub unsafe extern "C" fn _alloc_event(thrd: *mut pl330_thread) -> c_int {
    static inline int _alloc_event(struct pl330_thread *thrd)
    {
    struct pl330_dmac *pl330 = thrd.dmac;
    int ev;
    for (ev = 0; ev < pl330.pcfg.num_events; ev++)
    if (pl330.events[ev] == -1) {
    pl330.events[ev] = thrd.id;
    return ev;
    }
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn _chan_ns(pl330: *const pl330_dmac, i: c_int) -> bool {
    static bool _chan_ns(const struct pl330_dmac *pl330, int i)
    {
    return pl330.pcfg.irq_ns & (1 << i);
    }
// Upon success, returns IdentityToken for the
// allocated channel, NULL otherwise.
//
    static struct pl330_thread *pl330_request_channel(struct pl330_dmac *pl330)
    {
    struct pl330_thread *thrd = core::ptr::null_mut();
    int chans, i;
    if (pl330.state == DYING)
    return core::ptr::null_mut();
    chans = pl330.pcfg.num_chan;
    for (i = 0; i < chans; i++) {
    thrd = &pl330.channels[i];
    if ((thrd.free) && (!_manager_ns(thrd) ||
    _chan_ns(pl330, i))) {
    thrd.ev = _alloc_event(thrd);
    if (thrd.ev >= 0) {
    thrd.free = false;
    thrd.lstenq = 1;
    thrd.req[0].desc = core::ptr::null_mut();
    thrd.req[1].desc = core::ptr::null_mut();
    thrd.req_running = -1;
    break;
    }
    }
    thrd = core::ptr::null_mut();
    }
    return thrd;
    }
// Release an event
#[no_mangle]
pub unsafe extern "C" fn _free_event(thrd: *mut pl330_thread, ev: c_int) {
    static inline void _free_event(struct pl330_thread *thrd, int ev)
    {
    struct pl330_dmac *pl330 = thrd.dmac;
// If the event is valid and was held by the thread
    if (ev >= 0 && ev < pl330.pcfg.num_events
    && pl330.events[ev] == thrd.id)
    pl330.events[ev] = -1;
    }
#[no_mangle]
unsafe extern "C" fn pl330_release_channel(thrd: *mut pl330_thread) {
    static void pl330_release_channel(struct pl330_thread *thrd)
    {
    if (!thrd || thrd.free)
    return;
    _stop(thrd);
    dma_pl330_rqcb(thrd.req[1 - thrd.lstenq].desc, PL330_ERR_ABORT);
    dma_pl330_rqcb(thrd.req[thrd.lstenq].desc, PL330_ERR_ABORT);
    _free_event(thrd, thrd.ev);
    thrd.free = true;
    }
// Initialize the structure for PL330 configuration, that can be used
// by the client driver the make best use of the DMAC
//
#[no_mangle]
unsafe extern "C" fn read_dmac_config(pl330: *mut pl330_dmac) {
    static void read_dmac_config(struct pl330_dmac *pl330)
    {
    void __iomem *regs = pl330.base;
    u32 val;
    val = readl(regs + CRD) >> CRD_DATA_WIDTH_SHIFT;
    val &= CRD_DATA_WIDTH_MASK;
    pl330.pcfg.data_bus_width = 8 * (1 << val);
    val = readl(regs + CRD) >> CRD_DATA_BUFF_SHIFT;
    val &= CRD_DATA_BUFF_MASK;
    pl330.pcfg.data_buf_dep = val + 1;
    val = readl(regs + CR0) >> CR0_NUM_CHANS_SHIFT;
    val &= CR0_NUM_CHANS_MASK;
    val += 1;
    pl330.pcfg.num_chan = val;
    val = readl(regs + CR0);
    if (val & CR0_PERIPH_REQ_SET) {
    val = (val >> CR0_NUM_PERIPH_SHIFT) & CR0_NUM_PERIPH_MASK;
    val += 1;
    pl330.pcfg.num_peri = val;
    pl330.pcfg.peri_ns = readl(regs + CR4);
    } else {
    pl330.pcfg.num_peri = 0;
    }
    val = readl(regs + CR0);
    if (val & CR0_BOOT_MAN_NS)
    pl330.pcfg.mode |= DMAC_MODE_NS;
    else
    pl330.pcfg.mode &= ~DMAC_MODE_NS;
    val = readl(regs + CR0) >> CR0_NUM_EVENTS_SHIFT;
    val &= CR0_NUM_EVENTS_MASK;
    val += 1;
    pl330.pcfg.num_events = val;
    pl330.pcfg.irq_ns = readl(regs + CR3);
    }
#[no_mangle]
pub unsafe extern "C" fn _reset_thread(thrd: *mut pl330_thread) {
    static inline void _reset_thread(struct pl330_thread *thrd)
    {
    struct pl330_dmac *pl330 = thrd.dmac;
    thrd.req[0].mc_cpu = pl330.mcode_cpu
    + (thrd.id * pl330.mcbufsz);
    thrd.req[0].mc_bus = pl330.mcode_bus
    + (thrd.id * pl330.mcbufsz);
    thrd.req[0].desc = core::ptr::null_mut();
    thrd.req[1].mc_cpu = thrd.req[0].mc_cpu
    + pl330.mcbufsz / 2;
    thrd.req[1].mc_bus = thrd.req[0].mc_bus
    + pl330.mcbufsz / 2;
    thrd.req[1].desc = core::ptr::null_mut();
    thrd.req_running = -1;
    }
#[no_mangle]
unsafe extern "C" fn dmac_alloc_threads(pl330: *mut pl330_dmac) -> c_int {
    static int dmac_alloc_threads(struct pl330_dmac *pl330)
    {
    let mut chans: c_int = pl330.pcfg.num_chan;
    struct pl330_thread *thrd;
    int i;
// Allocate 1 Manager and 'chans' Channel threads
    pl330.channels = kzalloc_objs(*thrd, 1 + chans);
    if (!pl330.channels)
    return -ENOMEM;
// Init Channel threads
    for (i = 0; i < chans; i++) {
    thrd = &pl330.channels[i];
    thrd.id = i;
    thrd.dmac = pl330;
    _reset_thread(thrd);
    thrd.free = true;
    }
// MANAGER is indexed at the end
    thrd = &pl330.channels[chans];
    thrd.id = chans;
    thrd.dmac = pl330;
    thrd.free = false;
    pl330.manager = thrd;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dmac_alloc_resources(pl330: *mut pl330_dmac) -> c_int {
    static int dmac_alloc_resources(struct pl330_dmac *pl330)
    {
    let mut chans: c_int = pl330.pcfg.num_chan;
    int ret;
//
// Alloc MicroCode buffer for 'chans' Channel threads.
// A channel's buffer offset is (Channel_Id * MCODE_BUFF_PERCHAN)
//
    pl330.mcode_cpu = dma_alloc_attrs(pl330.ddma.dev,
    chans * pl330.mcbufsz,
    &pl330.mcode_bus, GFP_KERNEL,
    DMA_ATTR_PRIVILEGED);
    if (!pl330.mcode_cpu) {
    dev_err(pl330.ddma.dev, "%s:%d Can't allocate memory!\n",
    __func__, __LINE__);
    return -ENOMEM;
    }
    ret = dmac_alloc_threads(pl330);
    if (ret) {
    dev_err(pl330.ddma.dev, "%s:%d Can't to create channels for DMAC!\n",
    __func__, __LINE__);
    dma_free_attrs(pl330.ddma.dev,
    chans * pl330.mcbufsz,
    pl330.mcode_cpu, pl330.mcode_bus,
    DMA_ATTR_PRIVILEGED);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pl330_add(pl330: *mut pl330_dmac) -> c_int {
    static int pl330_add(struct pl330_dmac *pl330)
    {
    int i, ret;
// Check if we can handle this DMAC
    if ((pl330.pcfg.periph_id & 0xfffff) != PERIPH_ID_VAL) {
    dev_err(pl330.ddma.dev, "PERIPH_ID 0x%x !\n",
    pl330.pcfg.periph_id);
    return -EINVAL;
    }
// Read the configuration of the DMAC
    read_dmac_config(pl330);
    if (pl330.pcfg.num_events == 0) {
    dev_err(pl330.ddma.dev, "%s:%d Can't work without events!\n",
    __func__, __LINE__);
    return -EINVAL;
    }
    spin_lock_init(&pl330.lock);
    INIT_LIST_HEAD(&pl330.req_done);
// Use default MC buffer size if not provided
    if (!pl330.mcbufsz)
    pl330.mcbufsz = MCODE_BUFF_PER_REQ * 2;
// Mark all events as free
    for (i = 0; i < pl330.pcfg.num_events; i++)
    pl330.events[i] = -1;
// Allocate resources needed by the DMAC
    ret = dmac_alloc_resources(pl330);
    if (ret) {
    dev_err(pl330.ddma.dev, "Unable to create channels for DMAC\n");
    return ret;
    }
    tasklet_setup(&pl330.tasks, pl330_dotask);
    pl330.state = INIT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dmac_free_threads(pl330: *mut pl330_dmac) -> c_int {
    static int dmac_free_threads(struct pl330_dmac *pl330)
    {
    struct pl330_thread *thrd;
    int i;
// Release Channel threads
    for (i = 0; i < pl330.pcfg.num_chan; i++) {
    thrd = &pl330.channels[i];
    pl330_release_channel(thrd);
    }
// Free memory
    kfree(pl330.channels);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pl330_del(pl330: *mut pl330_dmac) {
    static void pl330_del(struct pl330_dmac *pl330)
    {
    pl330.state = UNINIT;
    tasklet_kill(&pl330.tasks);
// Free DMAC resources
    dmac_free_threads(pl330);
    dma_free_attrs(pl330.ddma.dev,
    pl330.pcfg.num_chan * pl330.mcbufsz, pl330.mcode_cpu,
    pl330.mcode_bus, DMA_ATTR_PRIVILEGED);
    }
// forward declaration
    static struct amba_driver pl330_driver;
    static inline struct dma_pl330_chan *
    to_pchan(struct dma_chan *ch)
    {
    if (!ch)
    return core::ptr::null_mut();
    return container_of(ch, struct dma_pl330_chan, chan);
    }
    static inline struct dma_pl330_desc *
    to_desc(struct dma_async_tx_descriptor *tx)
    {
    return container_of(tx, struct dma_pl330_desc, txd);
    }
#[no_mangle]
pub unsafe extern "C" fn fill_queue(pch: *mut dma_pl330_chan) {
    static inline void fill_queue(struct dma_pl330_chan *pch)
    {
    struct dma_pl330_desc *desc;
    int ret;
    list_for_each_entry(desc, &pch.work_list, node) {
// If already submitted
    if (desc.status == BUSY || desc.status == PAUSED)
    continue;
    ret = pl330_submit_req(pch.thread, desc);
    if (!ret) {
    desc.status = BUSY;
    } else if (ret == -EAGAIN) {
// QFull or DMAC Dying
    break;
    } else {
// Unacceptable request
    desc.status = DONE;
    dev_err(pch.dmac.ddma.dev, "%s:%d Bad Desc(%d)\n",
    __func__, __LINE__, desc.txd.cookie);
    tasklet_schedule(&pch.task);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn pl330_tasklet(t: *mut tasklet_struct) {
    static void pl330_tasklet(struct tasklet_struct *t)
    {
    struct dma_pl330_chan *pch = from_tasklet(pch, t, task);
    struct dma_pl330_desc *desc, *_dt;
    unsigned long flags;
    let mut power_down: bool = false;
    spin_lock_irqsave(&pch.lock, flags);
// Pick up ripe tomatoes
    list_for_each_entry_safe(desc, _dt, &pch.work_list, node)
    if (desc.status == DONE) {
    if (!pch.cyclic)
    dma_cookie_complete(&desc.txd);
    list_move_tail(&desc.node, &pch.completed_list);
    }
// Try to submit a req imm. next to the last completed cookie
    fill_queue(pch);
    if (list_empty(&pch.work_list)) {
    spin_lock(&pch.thread.dmac.lock);
    _stop(pch.thread);
    spin_unlock(&pch.thread.dmac.lock);
    power_down = true;
    pch.active = false;
    } else {
// Make sure the PL330 Channel thread is active
    spin_lock(&pch.thread.dmac.lock);
    pl330_start_thread(pch.thread);
    spin_unlock(&pch.thread.dmac.lock);
    }
    while (!list_empty(&pch.completed_list)) {
    struct dmaengine_desc_callback cb;
    desc = list_first_entry(&pch.completed_list,
    struct dma_pl330_desc, node);
    dmaengine_desc_get_callback(&desc.txd, &cb);
    if (pch.cyclic) {
    desc.status = PREP;
    list_move_tail(&desc.node, &pch.work_list);
    if (power_down) {
    pch.active = true;
    spin_lock(&pch.thread.dmac.lock);
    pl330_start_thread(pch.thread);
    spin_unlock(&pch.thread.dmac.lock);
    power_down = false;
    }
    } else {
    desc.status = FREE;
    list_move_tail(&desc.node, &pch.dmac.desc_pool);
    }
    dma_descriptor_unmap(&desc.txd);
    if (dmaengine_desc_callback_valid(&cb)) {
    spin_unlock_irqrestore(&pch.lock, flags);
    dmaengine_desc_callback_invoke(&cb, core::ptr::null_mut());
    spin_lock_irqsave(&pch.lock, flags);
    }
    }
    spin_unlock_irqrestore(&pch.lock, flags);
// If work list empty, power down
    if (power_down)
    pm_runtime_put_autosuspend(pch.dmac.ddma.dev);
    }
    static struct dma_chan *of_dma_pl330_xlate(struct of_phandle_args *dma_spec,
    struct of_dma *ofdma)
    {
    let mut count: c_int = dma_spec.args_count;
    struct pl330_dmac *pl330 = ofdma.of_dma_data;
    unsigned int chan_id;
    if (!pl330)
    return core::ptr::null_mut();
    if (count != 1)
    return core::ptr::null_mut();
    chan_id = dma_spec.args[0];
    if (chan_id >= pl330.num_peripherals)
    return core::ptr::null_mut();
    return dma_get_slave_channel(&pl330.peripherals[chan_id].chan);
    }
#[no_mangle]
unsafe extern "C" fn pl330_alloc_chan_resources(chan: *mut dma_chan) -> c_int {
    static int pl330_alloc_chan_resources(struct dma_chan *chan)
    {
    struct dma_pl330_chan *pch = to_pchan(chan);
    struct pl330_dmac *pl330 = pch.dmac;
    unsigned long flags;
    spin_lock_irqsave(&pl330.lock, flags);
    dma_cookie_init(chan);
    pch.cyclic = false;
    pch.thread = pl330_request_channel(pl330);
    if (!pch.thread) {
    spin_unlock_irqrestore(&pl330.lock, flags);
    return -ENOMEM;
    }
    tasklet_setup(&pch.task, pl330_tasklet);
    spin_unlock_irqrestore(&pl330.lock, flags);
    return 1;
    }
//
// We need the data direction between the DMAC (the dma-mapping "device") and
// the FIFO (the dmaengine "dev"), from the FIFO's point of view. Confusing!
//
    static enum dma_data_direction
    pl330_dma_slave_map_dir(enum dma_transfer_direction dir)
    {
    switch (dir) {
    case DMA_MEM_TO_DEV:
    return DMA_FROM_DEVICE;
    case DMA_DEV_TO_MEM:
    return DMA_TO_DEVICE;
    case DMA_DEV_TO_DEV:
    return DMA_BIDIRECTIONAL;
    default:
    return DMA_NONE;
    }
    }
#[no_mangle]
unsafe extern "C" fn pl330_unprep_slave_fifo(pch: *mut dma_pl330_chan) {
    static void pl330_unprep_slave_fifo(struct dma_pl330_chan *pch)
    {
    if (pch.dir != DMA_NONE)
    dma_unmap_resource(pch.chan.device.dev, pch.fifo_dma,
    1 << pch.burst_sz, pch.dir, 0);
    pch.dir = DMA_NONE;
    }
    static bool pl330_prep_slave_fifo(struct dma_pl330_chan *pch,
    enum dma_transfer_direction dir)
    {
    struct device *dev = pch.chan.device.dev;
    let mut dma_dir: enum dma_data_direction = pl330_dma_slave_map_dir(dir);
// Already mapped for this config?
    if (pch.dir == dma_dir)
    return true;
    pl330_unprep_slave_fifo(pch);
    pch.fifo_dma = dma_map_resource(dev, pch.fifo_addr,
    1 << pch.burst_sz, dma_dir, 0);
    if (dma_mapping_error(dev, pch.fifo_dma))
    return false;
    pch.dir = dma_dir;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn fixup_burst_len(max_burst_len: c_int, quirks: c_int) -> c_int {
    static int fixup_burst_len(int max_burst_len, int quirks)
    {
    if (max_burst_len > PL330_MAX_BURST)
    return PL330_MAX_BURST;
#[no_mangle]
pub unsafe extern "C" fn if(1: max_burst_len <) -> else {
    else if (max_burst_len < 1)
    return 1;
    else
    return max_burst_len;
    }
    static int pl330_config_write(struct dma_chan *chan,
    struct dma_slave_config *slave_config,
    enum dma_transfer_direction direction)
    {
    struct dma_pl330_chan *pch = to_pchan(chan);
    pl330_unprep_slave_fifo(pch);
    if (direction == DMA_MEM_TO_DEV) {
    if (slave_config.dst_addr)
    pch.fifo_addr = slave_config.dst_addr;
    if (slave_config.dst_addr_width)
    pch.burst_sz = __ffs(slave_config.dst_addr_width);
    pch.burst_len = fixup_burst_len(slave_config.dst_maxburst,
    pch.dmac.quirks);
    } else if (direction == DMA_DEV_TO_MEM) {
    if (slave_config.src_addr)
    pch.fifo_addr = slave_config.src_addr;
    if (slave_config.src_addr_width)
    pch.burst_sz = __ffs(slave_config.src_addr_width);
    pch.burst_len = fixup_burst_len(slave_config.src_maxburst,
    pch.dmac.quirks);
    }
    return 0;
    }
    static int pl330_config(struct dma_chan *chan,
    struct dma_slave_config *slave_config)
    {
    struct dma_pl330_chan *pch = to_pchan(chan);
    memcpy(&pch.slave_config, slave_config, sizeof(*slave_config));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pl330_terminate_all(chan: *mut dma_chan) -> c_int {
    static int pl330_terminate_all(struct dma_chan *chan)
    {
    struct dma_pl330_chan *pch = to_pchan(chan);
    struct dma_pl330_desc *desc;
    unsigned long flags;
    struct pl330_dmac *pl330 = pch.dmac;
    let mut power_down: bool = false;
    pm_runtime_get_sync(pl330.ddma.dev);
    spin_lock_irqsave(&pch.lock, flags);
    spin_lock(&pl330.lock);
    _stop(pch.thread);
    pch.thread.req[0].desc = core::ptr::null_mut();
    pch.thread.req[1].desc = core::ptr::null_mut();
    pch.thread.req_running = -1;
    spin_unlock(&pl330.lock);
    power_down = pch.active;
    pch.active = false;
// Mark all desc done
    list_for_each_entry(desc, &pch.submitted_list, node) {
    desc.status = FREE;
    dma_cookie_complete(&desc.txd);
    }
    list_for_each_entry(desc, &pch.work_list , node) {
    desc.status = FREE;
    dma_cookie_complete(&desc.txd);
    }
    list_splice_tail_init(&pch.submitted_list, &pl330.desc_pool);
    list_splice_tail_init(&pch.work_list, &pl330.desc_pool);
    list_splice_tail_init(&pch.completed_list, &pl330.desc_pool);
    spin_unlock_irqrestore(&pch.lock, flags);
    if (power_down)
    pm_runtime_put_autosuspend(pl330.ddma.dev);
    pm_runtime_put_autosuspend(pl330.ddma.dev);
    return 0;
    }
//
// We don't support DMA_RESUME command because of hardware
// limitations, so after pausing the channel we cannot restore
// it to active state. We have to terminate channel and setup
// DMA transfer again. This pause feature was implemented to
// allow safely read residue before channel termination.
//
#[no_mangle]
unsafe extern "C" fn pl330_pause(chan: *mut dma_chan) -> c_int {
    static int pl330_pause(struct dma_chan *chan)
    {
    struct dma_pl330_chan *pch = to_pchan(chan);
    struct pl330_dmac *pl330 = pch.dmac;
    struct dma_pl330_desc *desc;
    unsigned long flags;
    pm_runtime_get_sync(pl330.ddma.dev);
    spin_lock_irqsave(&pch.lock, flags);
    spin_lock(&pl330.lock);
    _stop(pch.thread);
    spin_unlock(&pl330.lock);
    list_for_each_entry(desc, &pch.work_list, node) {
    if (desc.status == BUSY)
    desc.status = PAUSED;
    }
    spin_unlock_irqrestore(&pch.lock, flags);
    pm_runtime_put_autosuspend(pl330.ddma.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pl330_free_chan_resources(chan: *mut dma_chan) {
    static void pl330_free_chan_resources(struct dma_chan *chan)
    {
    struct dma_pl330_chan *pch = to_pchan(chan);
    struct pl330_dmac *pl330 = pch.dmac;
    unsigned long flags;
    tasklet_kill(&pch.task);
    pm_runtime_get_sync(pch.dmac.ddma.dev);
    spin_lock_irqsave(&pl330.lock, flags);
    pl330_release_channel(pch.thread);
    pch.thread = core::ptr::null_mut();
    if (pch.cyclic)
    list_splice_tail_init(&pch.work_list, &pch.dmac.desc_pool);
    spin_unlock_irqrestore(&pl330.lock, flags);
    pm_runtime_put_autosuspend(pch.dmac.ddma.dev);
    pl330_unprep_slave_fifo(pch);
    }
    static int pl330_get_current_xferred_count(struct dma_pl330_chan *pch,
    struct dma_pl330_desc *desc)
    {
    struct pl330_thread *thrd = pch.thread;
    struct pl330_dmac *pl330 = pch.dmac;
    void __iomem *regs = thrd.dmac.base;
    u32 val, addr;
    pm_runtime_get_sync(pl330.ddma.dev);
    val = addr = 0;
    if (desc.rqcfg.src_inc) {
    val = readl(regs + SA(thrd.id));
    addr = desc.px.src_addr;
    } else {
    val = readl(regs + DA(thrd.id));
    addr = desc.px.dst_addr;
    }
    pm_runtime_mark_last_busy(pch.dmac.ddma.dev);
    pm_runtime_put_autosuspend(pl330.ddma.dev);
// If DMAMOV hasn't finished yet, SAR/DAR can be zero
    if (!val)
    return 0;
    return val - addr;
    }
    static enum dma_status
    pl330_tx_status(struct dma_chan *chan, dma_cookie_t cookie,
    struct dma_tx_state *txstate)
    {
    enum dma_status ret;
    unsigned long flags;
    struct dma_pl330_desc *desc, *running = core::ptr::null_mut(), *last_enq = core::ptr::null_mut();
    struct dma_pl330_chan *pch = to_pchan(chan);
    unsigned int transferred, residual = 0;
    ret = dma_cookie_status(chan, cookie, txstate);
    if (!txstate)
    return ret;
    if (ret == DMA_COMPLETE)
    goto out;
    spin_lock_irqsave(&pch.lock, flags);
    spin_lock(&pch.thread.dmac.lock);
    if (pch.thread.req_running != -1)
    running = pch.thread.req[pch.thread.req_running].desc;
    last_enq = pch.thread.req[pch.thread.lstenq].desc;
// Check in pending list
    list_for_each_entry(desc, &pch.work_list, node) {
    if (desc.status == DONE)
    transferred = desc.bytes_requested;
#[no_mangle]
pub unsafe extern "C" fn if(running: running && desc ==) -> else {
    else if (running && desc == running)
    transferred =
    pl330_get_current_xferred_count(pch, desc);
#[no_mangle]
pub unsafe extern "C" fn if(PAUSED: desc->status == BUSY || desc->status ==) -> else {
    else if (desc.status == BUSY || desc.status == PAUSED)
//
// Busy but not running means either just enqueued,
// or finished and not yet marked done
//
    if (desc == last_enq)
    transferred = 0;
    else
    transferred = desc.bytes_requested;
    else
    transferred = 0;
    residual += desc.bytes_requested - transferred;
    if (desc.txd.cookie == cookie) {
    switch (desc.status) {
    case DONE:
    ret = DMA_COMPLETE;
    break;
    case PAUSED:
    ret = DMA_PAUSED;
    break;
    case PREP:
    case BUSY:
    ret = DMA_IN_PROGRESS;
    break;
    default:
    WARN_ON(1);
    }
    break;
    }
    if (desc.last)
    residual = 0;
    }
    spin_unlock(&pch.thread.dmac.lock);
    spin_unlock_irqrestore(&pch.lock, flags);
    out:
    dma_set_residue(txstate, residual);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pl330_issue_pending(chan: *mut dma_chan) {
    static void pl330_issue_pending(struct dma_chan *chan)
    {
    struct dma_pl330_chan *pch = to_pchan(chan);
    unsigned long flags;
    spin_lock_irqsave(&pch.lock, flags);
    if (list_empty(&pch.work_list)) {
//
// Warn on nothing pending. Empty submitted_list may
// break our pm_runtime usage counter as it is
// updated on work_list emptiness status.
//
    WARN_ON(list_empty(&pch.submitted_list));
    pch.active = true;
    pm_runtime_get_sync(pch.dmac.ddma.dev);
    }
    list_splice_tail_init(&pch.submitted_list, &pch.work_list);
    spin_unlock_irqrestore(&pch.lock, flags);
    pl330_tasklet(&pch.task);
    }
//
// We returned the last one of the circular list of descriptor(s)
// from prep_xxx, so the argument to submit corresponds to the last
// descriptor of the list.
//
#[no_mangle]
unsafe extern "C" fn pl330_tx_submit(tx: *mut dma_async_tx_descriptor) -> dma_cookie_t {
    static dma_cookie_t pl330_tx_submit(struct dma_async_tx_descriptor *tx)
    {
    struct dma_pl330_desc *desc, *last = to_desc(tx);
    struct dma_pl330_chan *pch = to_pchan(tx.chan);
    dma_cookie_t cookie;
    unsigned long flags;
    spin_lock_irqsave(&pch.lock, flags);
// Assign cookies to all nodes
    while (!list_empty(&last.node)) {
    desc = list_entry(last.node.next, struct dma_pl330_desc, node);
    if (pch.cyclic) {
    desc.txd.callback = last.txd.callback;
    desc.txd.callback_param = last.txd.callback_param;
    }
    desc.last = false;
    dma_cookie_assign(&desc.txd);
    list_move_tail(&desc.node, &pch.submitted_list);
    }
    last.last = true;
    cookie = dma_cookie_assign(&last.txd);
    list_add_tail(&last.node, &pch.submitted_list);
    spin_unlock_irqrestore(&pch.lock, flags);
    return cookie;
    }
#[no_mangle]
pub unsafe extern "C" fn _init_desc(desc: *mut dma_pl330_desc) {
    static inline void _init_desc(struct dma_pl330_desc *desc)
    {
    desc.rqcfg.swap = SWAP_NO;
    desc.rqcfg.scctl = CCTRL0;
    desc.rqcfg.dcctl = CCTRL0;
    desc.txd.tx_submit = pl330_tx_submit;
    INIT_LIST_HEAD(&desc.node);
    }
// Returns the number of descriptors added to the DMAC pool
    static int add_desc(struct list_head *pool, spinlock_t *lock,
    gfp_t flg, int count)
    {
    struct dma_pl330_desc *desc;
    unsigned long flags;
    int i;
    desc = kzalloc_objs(*desc, count, flg);
    if (!desc)
    return 0;
    spin_lock_irqsave(lock, flags);
    for (i = 0; i < count; i++) {
    _init_desc(&desc[i]);
    list_add_tail(&desc[i].node, pool);
    }
    spin_unlock_irqrestore(lock, flags);
    return count;
    }
    static struct dma_pl330_desc *pluck_desc(struct list_head *pool,
    spinlock_t *lock)
    {
    struct dma_pl330_desc *desc = core::ptr::null_mut();
    unsigned long flags;
    spin_lock_irqsave(lock, flags);
    if (!list_empty(pool)) {
    desc = list_entry(pool.next,
    struct dma_pl330_desc, node);
    list_del_init(&desc.node);
    desc.status = PREP;
    desc.txd.callback = core::ptr::null_mut();
    desc.txd.callback_result = core::ptr::null_mut();
    }
    spin_unlock_irqrestore(lock, flags);
    return desc;
    }
    static struct dma_pl330_desc *pl330_get_desc(struct dma_pl330_chan *pch)
    {
    struct pl330_dmac *pl330 = pch.dmac;
    u8 *peri_id = pch.chan.private;
    struct dma_pl330_desc *desc;
// Pluck one desc from the pool of DMAC
    desc = pluck_desc(&pl330.desc_pool, &pl330.pool_lock);
// If the DMAC pool is empty, alloc new
    if (!desc) {
    static DEFINE_SPINLOCK(lock);
    LIST_HEAD(pool);
    if (!add_desc(&pool, &lock, GFP_ATOMIC, 1))
    return core::ptr::null_mut();
    desc = pluck_desc(&pool, &lock);
    WARN_ON(!desc || !list_empty(&pool));
    }
// Initialize the descriptor
    desc.pchan = pch;
    desc.txd.cookie = 0;
    async_tx_ack(&desc.txd);
    desc.peri = peri_id ? pch.chan.chan_id : 0;
    desc.rqcfg.pcfg = &pch.dmac.pcfg;
    dma_async_tx_descriptor_init(&desc.txd, &pch.chan);
    return desc;
    }
    static inline void fill_px(struct pl330_xfer *px,
    dma_addr_t dst, dma_addr_t src, size_t len)
    {
    px.bytes = len;
    px.dst_addr = dst;
    px.src_addr = src;
    }
    static struct dma_pl330_desc *
    __pl330_prep_dma_memcpy(struct dma_pl330_chan *pch, dma_addr_t dst,
    dma_addr_t src, size_t len)
    {
    struct dma_pl330_desc *desc = pl330_get_desc(pch);
    if (!desc) {
    dev_err(pch.dmac.ddma.dev, "%s:%d Unable to fetch desc\n",
    __func__, __LINE__);
    return core::ptr::null_mut();
    }
//
// Ideally we should lookout for reqs bigger than
// those that can be programmed with 256 bytes of
// MC buffer, but considering a req size is seldom
// going to be word-unaligned and more than 200MB,
// we take it easy.
// Also, should the limit is reached we'd rather
// have the platform increase MC buffer size than
// complicating this API driver.
//
    fill_px(&desc.px, dst, src, len);
    return desc;
    }
// Call after fixing burst size
#[no_mangle]
pub unsafe extern "C" fn get_burst_len(desc: *mut dma_pl330_desc, len: usize) -> c_int {
    static inline int get_burst_len(struct dma_pl330_desc *desc, size_t len)
    {
    struct dma_pl330_chan *pch = desc.pchan;
    struct pl330_dmac *pl330 = pch.dmac;
    int burst_len;
    burst_len = pl330.pcfg.data_bus_width / 8;
    burst_len *= pl330.pcfg.data_buf_dep / pl330.pcfg.num_chan;
    burst_len >>= desc.rqcfg.brst_size;
// src/dst_burst_len can't be more than 16
    if (burst_len > PL330_MAX_BURST)
    burst_len = PL330_MAX_BURST;
    return burst_len;
    }
    static struct dma_async_tx_descriptor *pl330_prep_dma_cyclic(
    struct dma_chan *chan, dma_addr_t dma_addr, size_t len,
    size_t period_len, enum dma_transfer_direction direction,
    unsigned long flags)
    {
    struct dma_pl330_desc *desc = core::ptr::null_mut(), *first = core::ptr::null_mut();
    struct dma_pl330_chan *pch = to_pchan(chan);
    struct pl330_dmac *pl330 = pch.dmac;
    unsigned int i;
    dma_addr_t dst;
    dma_addr_t src;
    if (len % period_len != 0)
    return core::ptr::null_mut();
    if (!is_slave_direction(direction)) {
    dev_err(pch.dmac.ddma.dev, "%s:%d Invalid dma direction\n",
    __func__, __LINE__);
    return core::ptr::null_mut();
    }
    pl330_config_write(chan, &pch.slave_config, direction);
    if (!pl330_prep_slave_fifo(pch, direction))
    return core::ptr::null_mut();
    for (i = 0; i < len / period_len; i++) {
    desc = pl330_get_desc(pch);
    if (!desc) {
    unsigned long iflags;
    dev_err(pch.dmac.ddma.dev, "%s:%d Unable to fetch desc\n",
    __func__, __LINE__);
    if (!first)
    return core::ptr::null_mut();
    spin_lock_irqsave(&pl330.pool_lock, iflags);
    while (!list_empty(&first.node)) {
    desc = list_entry(first.node.next,
    struct dma_pl330_desc, node);
    list_move_tail(&desc.node, &pl330.desc_pool);
    }
    list_move_tail(&first.node, &pl330.desc_pool);
    spin_unlock_irqrestore(&pl330.pool_lock, iflags);
    return core::ptr::null_mut();
    }
    switch (direction) {
    case DMA_MEM_TO_DEV:
    desc.rqcfg.src_inc = 1;
    desc.rqcfg.dst_inc = 0;
    src = dma_addr;
    dst = pch.fifo_dma;
    break;
    case DMA_DEV_TO_MEM:
    desc.rqcfg.src_inc = 0;
    desc.rqcfg.dst_inc = 1;
    src = pch.fifo_dma;
    dst = dma_addr;
    break;
    default:
    break;
    }
    desc.rqtype = direction;
    desc.rqcfg.brst_size = pch.burst_sz;
    desc.rqcfg.brst_len = pch.burst_len;
    desc.bytes_requested = period_len;
    fill_px(&desc.px, dst, src, period_len);
    if (!first)
    first = desc;
    else
    list_add_tail(&desc.node, &first.node);
    dma_addr += period_len;
    }
    if (!desc)
    return core::ptr::null_mut();
    pch.cyclic = true;
    return &desc.txd;
    }
    static struct dma_async_tx_descriptor *
    pl330_prep_dma_memcpy(struct dma_chan *chan, dma_addr_t dst,
    dma_addr_t src, size_t len, unsigned long flags)
    {
    struct dma_pl330_desc *desc;
    struct dma_pl330_chan *pch = to_pchan(chan);
    struct pl330_dmac *pl330;
    int burst;
    if (unlikely(!pch || !len))
    return core::ptr::null_mut();
    pl330 = pch.dmac;
    desc = __pl330_prep_dma_memcpy(pch, dst, src, len);
    if (!desc)
    return core::ptr::null_mut();
    desc.rqcfg.src_inc = 1;
    desc.rqcfg.dst_inc = 1;
    desc.rqtype = DMA_MEM_TO_MEM;
// Select max possible burst size
    burst = pl330.pcfg.data_bus_width / 8;
//
// Make sure we use a burst size that aligns with all the memcpy
// parameters because our DMA programming algorithm doesn't cope with
// transfers which straddle an entry in the DMA device's MFIFO.
//
    while ((src | dst | len) & (burst - 1))
    burst /= 2;
    desc.rqcfg.brst_size = 0;
    while (burst != (1 << desc.rqcfg.brst_size))
    desc.rqcfg.brst_size++;
    desc.rqcfg.brst_len = get_burst_len(desc, len);
//
// If burst size is smaller than bus width then make sure we only
// transfer one at a time to avoid a burst stradling an MFIFO entry.
//
    if (burst * 8 < pl330.pcfg.data_bus_width)
    desc.rqcfg.brst_len = 1;
    desc.bytes_requested = len;
    return &desc.txd;
    }
    static void __pl330_giveback_desc(struct pl330_dmac *pl330,
    struct dma_pl330_desc *first)
    {
    unsigned long flags;
    struct dma_pl330_desc *desc;
    if (!first)
    return;
    spin_lock_irqsave(&pl330.pool_lock, flags);
    while (!list_empty(&first.node)) {
    desc = list_entry(first.node.next,
    struct dma_pl330_desc, node);
    list_move_tail(&desc.node, &pl330.desc_pool);
    }
    list_move_tail(&first.node, &pl330.desc_pool);
    spin_unlock_irqrestore(&pl330.pool_lock, flags);
    }
    static struct dma_async_tx_descriptor *
    pl330_prep_slave_sg(struct dma_chan *chan, struct scatterlist *sgl,
    unsigned int sg_len, enum dma_transfer_direction direction,
    unsigned long flg, void *context)
    {
    struct dma_pl330_desc *first, *desc = core::ptr::null_mut();
    struct dma_pl330_chan *pch = to_pchan(chan);
    struct scatterlist *sg;
    int i;
    if (unlikely(!pch || !sgl || !sg_len))
    return core::ptr::null_mut();
    pl330_config_write(chan, &pch.slave_config, direction);
    if (!pl330_prep_slave_fifo(pch, direction))
    return core::ptr::null_mut();
    first = core::ptr::null_mut();
    for_each_sg(sgl, sg, sg_len, i) {
    desc = pl330_get_desc(pch);
    if (!desc) {
    struct pl330_dmac *pl330 = pch.dmac;
    dev_err(pch.dmac.ddma.dev,
    "%s:%d Unable to fetch desc\n",
    __func__, __LINE__);
    __pl330_giveback_desc(pl330, first);
    return core::ptr::null_mut();
    }
    if (!first)
    first = desc;
    else
    list_add_tail(&desc.node, &first.node);
    if (direction == DMA_MEM_TO_DEV) {
    desc.rqcfg.src_inc = 1;
    desc.rqcfg.dst_inc = 0;
    fill_px(&desc.px, pch.fifo_dma, sg_dma_address(sg),
    sg_dma_len(sg));
    } else {
    desc.rqcfg.src_inc = 0;
    desc.rqcfg.dst_inc = 1;
    fill_px(&desc.px, sg_dma_address(sg), pch.fifo_dma,
    sg_dma_len(sg));
    }
    desc.rqcfg.brst_size = pch.burst_sz;
    desc.rqcfg.brst_len = pch.burst_len;
    desc.rqtype = direction;
    desc.bytes_requested = sg_dma_len(sg);
    }
// Return the last desc in the chain
    return &desc.txd;
    }
#[no_mangle]
unsafe extern "C" fn pl330_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t pl330_irq_handler(int irq, void *data)
    {
    if (pl330_update(data))
    return IRQ_HANDLED;
    else
    return IRQ_NONE;
    }

    BIT(DMA_SLAVE_BUSWIDTH_UNDEFINED) | \
    BIT(DMA_SLAVE_BUSWIDTH_1_BYTE) | \
    BIT(DMA_SLAVE_BUSWIDTH_2_BYTES) | \
    BIT(DMA_SLAVE_BUSWIDTH_4_BYTES) | \
    BIT(DMA_SLAVE_BUSWIDTH_8_BYTES)

#[no_mangle]
unsafe extern "C" fn pl330_debugfs_show(s: *mut seq_file, data: *mut c_void) -> c_int {
    static int pl330_debugfs_show(struct seq_file *s, void *data)
    {
    struct pl330_dmac *pl330 = s.private;
    int chans, pchs, ch, pr;
    chans = pl330.pcfg.num_chan;
    pchs = pl330.num_peripherals;
    seq_puts(s, "PL330 physical channels:\n");
    seq_puts(s, "THREAD:\t\tCHANNEL:\n");
    seq_puts(s, "--------\t-----\n");
    for (ch = 0; ch < chans; ch++) {
    struct pl330_thread *thrd = &pl330.channels[ch];
    let mut found: c_int = -1;
    for (pr = 0; pr < pchs; pr++) {
    struct dma_pl330_chan *pch = &pl330.peripherals[pr];
    if (!pch.thread || thrd.id != pch.thread.id)
    continue;
    found = pr;
    }
    seq_printf(s, "%d\t\t", thrd.id);
    if (found == -1)
    seq_puts(s, "--\n");
    else
    seq_printf(s, "%d\n", found);
    }
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(pl330_debugfs);
#[no_mangle]
pub unsafe extern "C" fn init_pl330_debugfs(pl330: *mut pl330_dmac) {
    static inline void init_pl330_debugfs(struct pl330_dmac *pl330)
    {
    pl330.dbgfs = debugfs_create_file(dev_name(pl330.ddma.dev),
    S_IFREG | 0444, core::ptr::null_mut(), pl330,
    &pl330_debugfs_fops);
    }
#[no_mangle]
pub unsafe extern "C" fn deinit_pl330_debugfs(pl330: *mut pl330_dmac) {
    static inline void deinit_pl330_debugfs(struct pl330_dmac *pl330)
    {
    debugfs_remove(pl330.dbgfs);
    pl330.dbgfs = core::ptr::null_mut();
    }

#[no_mangle]
pub unsafe extern "C" fn init_pl330_debugfs(pl330: *mut pl330_dmac) {
    static inline void init_pl330_debugfs(struct pl330_dmac *pl330)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn deinit_pl330_debugfs(pl330: *mut pl330_dmac) {
    static inline void deinit_pl330_debugfs(struct pl330_dmac *pl330)
    {
    }

//
// Runtime PM callbacks are provided by amba/bus.c driver.
//
// It is assumed here that IRQ safe runtime PM is chosen in probe and amba
// bus driver will only disable/enable the clock in runtime PM callbacks.
//
#[no_mangle]
unsafe extern "C" fn pl330_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused pl330_suspend(struct device *dev)
    {
    struct amba_device *pcdev = to_amba_device(dev);
    pm_runtime_force_suspend(dev);
    clk_unprepare(pcdev.pclk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pl330_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused pl330_resume(struct device *dev)
    {
    struct amba_device *pcdev = to_amba_device(dev);
    int ret;
    ret = clk_prepare(pcdev.pclk);
    if (ret)
    return ret;
    pm_runtime_force_resume(dev);
    return ret;
    }
    static const struct dev_pm_ops pl330_pm = {
    SET_LATE_SYSTEM_SLEEP_PM_OPS(pl330_suspend, pl330_resume)
    };
    static int
    pl330_probe(struct amba_device *adev, const struct amba_id *id)
    {
    struct pl330_config *pcfg;
    struct pl330_dmac *pl330;
    struct dma_pl330_chan *pch, *_p;
    struct dma_device *pd;
    struct resource *res;
    int i, ret, irq;
    int num_chan;
    struct device_node *np = adev.dev.of_node;
    ret = dma_set_mask_and_coherent(&adev.dev, DMA_BIT_MASK(32));
    if (ret)
    return ret;
// Allocate a new DMAC and its Channels
    pl330 = devm_kzalloc(&adev.dev, sizeof(*pl330), GFP_KERNEL);
    if (!pl330)
    return -ENOMEM;
    pd = &pl330.ddma;
    pd.dev = &adev.dev;
    pl330.mcbufsz = 0;
// get quirk
    for (i = 0; i < ARRAY_SIZE(of_quirks); i++)
    if (of_property_read_bool(np, of_quirks[i].quirk))
    pl330.quirks |= of_quirks[i].id;
    res = &adev.res;
    pl330.base = devm_ioremap_resource(&adev.dev, res);
    if (IS_ERR(pl330.base))
    return PTR_ERR(pl330.base);
    amba_set_drvdata(adev, pl330);
    pl330.rstc = devm_reset_control_get_optional(&adev.dev, "dma");
    if (IS_ERR(pl330.rstc)) {
    return dev_err_probe(&adev.dev, PTR_ERR(pl330.rstc), "Failed to get reset!\n");
    } else {
    ret = reset_control_deassert(pl330.rstc);
    if (ret) {
    dev_err(&adev.dev, "Couldn't deassert the device from reset!\n");
    return ret;
    }
    }
    pl330.rstc_ocp = devm_reset_control_get_optional(&adev.dev, "dma-ocp");
    if (IS_ERR(pl330.rstc_ocp)) {
    return dev_err_probe(&adev.dev, PTR_ERR(pl330.rstc_ocp),
    "Failed to get OCP reset!\n");
    } else {
    ret = reset_control_deassert(pl330.rstc_ocp);
    if (ret) {
    dev_err(&adev.dev, "Couldn't deassert the device from OCP reset!\n");
    return ret;
    }
    }
    for (i = 0; i < AMBA_NR_IRQS; i++) {
    irq = adev.irq[i];
    if (irq) {
    ret = devm_request_irq(&adev.dev, irq,
    pl330_irq_handler, 0,
    dev_name(&adev.dev), pl330);
    if (ret)
    return ret;
    } else {
    break;
    }
    }
    pcfg = &pl330.pcfg;
    pcfg.periph_id = adev.periphid;
    ret = pl330_add(pl330);
    if (ret)
    return ret;
    INIT_LIST_HEAD(&pl330.desc_pool);
    spin_lock_init(&pl330.pool_lock);
// Create a descriptor pool of default size
    if (!add_desc(&pl330.desc_pool, &pl330.pool_lock,
    GFP_KERNEL, NR_DEFAULT_DESC))
    dev_warn(&adev.dev, "unable to allocate desc\n");
    INIT_LIST_HEAD(&pd.channels);
// Initialize channel parameters
    num_chan = max_t(int, pcfg.num_peri, pcfg.num_chan);
    pl330.num_peripherals = num_chan;
    pl330.peripherals = kzalloc_objs(*pch, num_chan);
    if (!pl330.peripherals) {
    ret = -ENOMEM;
    goto probe_err2;
    }
    for (i = 0; i < num_chan; i++) {
    pch = &pl330.peripherals[i];
    pch.chan.private = adev.dev.of_node;
    INIT_LIST_HEAD(&pch.submitted_list);
    INIT_LIST_HEAD(&pch.work_list);
    INIT_LIST_HEAD(&pch.completed_list);
    spin_lock_init(&pch.lock);
    pch.thread = core::ptr::null_mut();
    pch.chan.device = pd;
    pch.dmac = pl330;
    pch.dir = DMA_NONE;
// Add the channel to the DMAC list
    list_add_tail(&pch.chan.device_node, &pd.channels);
    }
    dma_cap_set(DMA_MEMCPY, pd.cap_mask);
    if (pcfg.num_peri) {
    dma_cap_set(DMA_SLAVE, pd.cap_mask);
    dma_cap_set(DMA_CYCLIC, pd.cap_mask);
    dma_cap_set(DMA_PRIVATE, pd.cap_mask);
    }
    pd.device_alloc_chan_resources = pl330_alloc_chan_resources;
    pd.device_free_chan_resources = pl330_free_chan_resources;
    pd.device_prep_dma_memcpy = pl330_prep_dma_memcpy;
    pd.device_prep_dma_cyclic = pl330_prep_dma_cyclic;
    pd.device_tx_status = pl330_tx_status;
    pd.device_prep_slave_sg = pl330_prep_slave_sg;
    pd.device_config = pl330_config;
    pd.device_pause = pl330_pause;
    pd.device_terminate_all = pl330_terminate_all;
    pd.device_issue_pending = pl330_issue_pending;
    pd.src_addr_widths = PL330_DMA_BUSWIDTHS;
    pd.dst_addr_widths = PL330_DMA_BUSWIDTHS;
    pd.directions = BIT(DMA_DEV_TO_MEM) | BIT(DMA_MEM_TO_DEV);
    pd.residue_granularity = DMA_RESIDUE_GRANULARITY_BURST;
    pd.max_burst = PL330_MAX_BURST;
    ret = dma_async_device_register(pd);
    if (ret) {
    dev_err(&adev.dev, "unable to register DMAC\n");
    goto probe_err3;
    }
    if (adev.dev.of_node) {
    ret = of_dma_controller_register(adev.dev.of_node,
    of_dma_pl330_xlate, pl330);
    if (ret) {
    dev_err(&adev.dev,
    "unable to register DMA to the generic DT DMA helpers\n");
    }
    }
//
// This is the limit for transfers with a buswidth of 1, larger
// buswidths will have larger limits.
//
    dma_set_max_seg_size(&adev.dev, 1900800);
    init_pl330_debugfs(pl330);
    dev_info(&adev.dev,
    "Loaded driver for PL330 DMAC-%x\n", adev.periphid);
    dev_info(&adev.dev,
    "\tDBUFF-%ux%ubytes Num_Chans-%u Num_Peri-%u Num_Events-%u\n",
    pcfg.data_buf_dep, pcfg.data_bus_width / 8, pcfg.num_chan,
    pcfg.num_peri, pcfg.num_events);
    pm_runtime_irq_safe(&adev.dev);
    pm_runtime_use_autosuspend(&adev.dev);
    pm_runtime_set_autosuspend_delay(&adev.dev, PL330_AUTOSUSPEND_DELAY);
    pm_runtime_put_autosuspend(&adev.dev);
    return 0;
    probe_err3:
// Idle the DMAC
    list_for_each_entry_safe(pch, _p, &pl330.ddma.channels,
    chan.device_node) {
// Remove the channel
    list_del(&pch.chan.device_node);
// Flush the channel
    if (pch.thread) {
    pl330_terminate_all(&pch.chan);
    pl330_free_chan_resources(&pch.chan);
    }
    }
    probe_err2:
    pl330_del(pl330);
    if (pl330.rstc_ocp)
    reset_control_assert(pl330.rstc_ocp);
    if (pl330.rstc)
    reset_control_assert(pl330.rstc);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pl330_remove(adev: *mut amba_device) {
    static void pl330_remove(struct amba_device *adev)
    {
    struct pl330_dmac *pl330 = amba_get_drvdata(adev);
    struct dma_pl330_chan *pch, *_p;
    int i, irq;
    deinit_pl330_debugfs(pl330);
    pm_runtime_get_noresume(pl330.ddma.dev);
    if (adev.dev.of_node)
    of_dma_controller_free(adev.dev.of_node);
    for (i = 0; i < AMBA_NR_IRQS; i++) {
    irq = adev.irq[i];
    if (irq)
    devm_free_irq(&adev.dev, irq, pl330);
    }
    dma_async_device_unregister(&pl330.ddma);
// Idle the DMAC
    list_for_each_entry_safe(pch, _p, &pl330.ddma.channels,
    chan.device_node) {
// Remove the channel
    list_del(&pch.chan.device_node);
// Flush the channel
    if (pch.thread) {
    pl330_terminate_all(&pch.chan);
    pl330_free_chan_resources(&pch.chan);
    }
    }
    pl330_del(pl330);
    if (pl330.rstc_ocp)
    reset_control_assert(pl330.rstc_ocp);
    if (pl330.rstc)
    reset_control_assert(pl330.rstc);
    }
    static const struct amba_id pl330_ids[] = {
    {
    .id	= 0x00041330,
    .mask	= 0x000fffff,
    },
    { 0, 0 },
    };
    MODULE_DEVICE_TABLE(amba, pl330_ids);
    static struct amba_driver pl330_driver = {
    .drv = {
    .name = "dma-pl330",
    .pm = &pl330_pm,
    },
    .id_table = pl330_ids,
    .probe = pl330_probe,
    .remove = pl330_remove,
    };
    module_amba_driver(pl330_driver);
    MODULE_AUTHOR("Jaswinder Singh <jassisinghbrar@gmail.com>");
    MODULE_DESCRIPTION("API Driver for PL330 DMAC");
    MODULE_LICENSE("GPL");
