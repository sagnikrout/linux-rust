//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb3/regs.h
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
pub const A_SG_CONTROL: c_uint = 0x0;
pub const S_CONGMODE: c_int = 29;

pub const S_TNLFLMODE: c_int = 28;

pub const S_FATLPERREN: c_int = 27;

pub const S_DROPPKT: c_int = 20;

pub const S_EGRGENCTRL: c_int = 19;

pub const S_USERSPACESIZE: c_int = 14;
pub const M_USERSPACESIZE: c_uint = 0x1f;

pub const S_HOSTPAGESIZE: c_int = 11;
pub const M_HOSTPAGESIZE: c_uint = 0x7;

pub const S_FLMODE: c_int = 9;

pub const S_PKTSHIFT: c_int = 6;
pub const M_PKTSHIFT: c_uint = 0x7;

pub const S_ONEINTMULTQ: c_int = 5;

pub const S_BIGENDIANINGRESS: c_int = 2;

pub const S_ISCSICOALESCING: c_int = 1;

pub const S_GLOBALENABLE: c_int = 0;

pub const S_AVOIDCQOVFL: c_int = 24;

pub const S_OPTONEINTMULTQ: c_int = 23;

pub const S_CQCRDTCTRL: c_int = 22;

pub const A_SG_KDOORBELL: c_uint = 0x4;
pub const S_SELEGRCNTX: c_int = 31;

pub const S_EGRCNTX: c_int = 0;
pub const M_EGRCNTX: c_uint = 0xffff;

pub const A_SG_GTS: c_uint = 0x8;
pub const S_RSPQ: c_int = 29;
pub const M_RSPQ: c_uint = 0x7;

pub const S_NEWTIMER: c_int = 16;
pub const M_NEWTIMER: c_uint = 0x1fff;

pub const S_NEWINDEX: c_int = 0;
pub const M_NEWINDEX: c_uint = 0xffff;

pub const A_SG_CONTEXT_CMD: c_uint = 0xc;
pub const S_CONTEXT_CMD_OPCODE: c_int = 28;
pub const M_CONTEXT_CMD_OPCODE: c_uint = 0xf;

pub const S_CONTEXT_CMD_BUSY: c_int = 27;

pub const S_CQ_CREDIT: c_int = 20;
pub const M_CQ_CREDIT: c_uint = 0x7f;

pub const S_CQ: c_int = 19;

pub const S_RESPONSEQ: c_int = 18;

pub const S_EGRESS: c_int = 17;

pub const S_FREELIST: c_int = 16;

pub const S_CONTEXT: c_int = 0;
pub const M_CONTEXT: c_uint = 0xffff;

pub const A_SG_CONTEXT_DATA0: c_uint = 0x10;
pub const A_SG_CONTEXT_DATA1: c_uint = 0x14;
pub const A_SG_CONTEXT_DATA2: c_uint = 0x18;
pub const A_SG_CONTEXT_DATA3: c_uint = 0x1c;
pub const A_SG_CONTEXT_MASK0: c_uint = 0x20;
pub const A_SG_CONTEXT_MASK1: c_uint = 0x24;
pub const A_SG_CONTEXT_MASK2: c_uint = 0x28;
pub const A_SG_CONTEXT_MASK3: c_uint = 0x2c;
pub const A_SG_RSPQ_CREDIT_RETURN: c_uint = 0x30;
pub const S_CREDITS: c_int = 0;
pub const M_CREDITS: c_uint = 0xffff;

pub const A_SG_DATA_INTR: c_uint = 0x34;
pub const S_ERRINTR: c_int = 31;

pub const A_SG_HI_DRB_HI_THRSH: c_uint = 0x38;
pub const A_SG_HI_DRB_LO_THRSH: c_uint = 0x3c;
pub const A_SG_LO_DRB_HI_THRSH: c_uint = 0x40;
pub const A_SG_LO_DRB_LO_THRSH: c_uint = 0x44;
pub const A_SG_RSPQ_FL_STATUS: c_uint = 0x4c;
pub const S_RSPQ0DISABLED: c_int = 8;
pub const S_FL0EMPTY: c_int = 16;

pub const A_SG_EGR_RCQ_DRB_THRSH: c_uint = 0x54;
pub const S_HIRCQDRBTHRSH: c_int = 16;
pub const M_HIRCQDRBTHRSH: c_uint = 0x7ff;

pub const S_LORCQDRBTHRSH: c_int = 0;
pub const M_LORCQDRBTHRSH: c_uint = 0x7ff;

pub const A_SG_EGR_CNTX_BADDR: c_uint = 0x58;
pub const A_SG_INT_CAUSE: c_uint = 0x5c;
pub const S_HIRCQPARITYERROR: c_int = 31;

pub const S_LORCQPARITYERROR: c_int = 30;

pub const S_HIDRBPARITYERROR: c_int = 29;

pub const S_LODRBPARITYERROR: c_int = 28;

pub const S_FLPARITYERROR: c_int = 22;
pub const M_FLPARITYERROR: c_uint = 0x3f;

pub const S_ITPARITYERROR: c_int = 20;
pub const M_ITPARITYERROR: c_uint = 0x3;

pub const S_IRPARITYERROR: c_int = 19;

pub const S_RCPARITYERROR: c_int = 18;

pub const S_OCPARITYERROR: c_int = 17;

pub const S_CPPARITYERROR: c_int = 16;

pub const S_R_REQ_FRAMINGERROR: c_int = 15;

pub const S_UC_REQ_FRAMINGERROR: c_int = 14;

pub const S_HICTLDRBDROPERR: c_int = 13;

pub const S_LOCTLDRBDROPERR: c_int = 12;

pub const S_HIPIODRBDROPERR: c_int = 11;

pub const S_LOPIODRBDROPERR: c_int = 10;

pub const S_HIPRIORITYDBFULL: c_int = 7;

pub const S_HIPRIORITYDBEMPTY: c_int = 6;

pub const S_LOPRIORITYDBFULL: c_int = 5;

pub const S_LOPRIORITYDBEMPTY: c_int = 4;

pub const S_RSPQDISABLED: c_int = 3;

pub const S_RSPQCREDITOVERFOW: c_int = 2;

pub const S_FLEMPTY: c_int = 1;

pub const A_SG_INT_ENABLE: c_uint = 0x60;
pub const A_SG_CMDQ_CREDIT_TH: c_uint = 0x64;
pub const S_TIMEOUT: c_int = 8;
pub const M_TIMEOUT: c_uint = 0xffffff;

pub const S_THRESHOLD: c_int = 0;
pub const M_THRESHOLD: c_uint = 0xff;

pub const A_SG_TIMER_TICK: c_uint = 0x68;
pub const A_SG_CQ_CONTEXT_BADDR: c_uint = 0x6c;
pub const A_SG_OCO_BASE: c_uint = 0x70;
pub const S_BASE1: c_int = 16;
pub const M_BASE1: c_uint = 0xffff;

pub const A_SG_DRB_PRI_THRESH: c_uint = 0x74;
pub const A_PCIX_INT_ENABLE: c_uint = 0x80;
pub const S_MSIXPARERR: c_int = 22;
pub const M_MSIXPARERR: c_uint = 0x7;

pub const S_CFPARERR: c_int = 18;
pub const M_CFPARERR: c_uint = 0xf;

pub const S_RFPARERR: c_int = 14;
pub const M_RFPARERR: c_uint = 0xf;

pub const S_WFPARERR: c_int = 12;
pub const M_WFPARERR: c_uint = 0x3;

pub const S_PIOPARERR: c_int = 11;

pub const S_DETUNCECCERR: c_int = 10;

pub const S_DETCORECCERR: c_int = 9;

pub const S_RCVSPLCMPERR: c_int = 8;

pub const S_UNXSPLCMP: c_int = 7;

pub const S_SPLCMPDIS: c_int = 6;

pub const S_DETPARERR: c_int = 5;

pub const S_SIGSYSERR: c_int = 4;

pub const S_RCVMSTABT: c_int = 3;

pub const S_RCVTARABT: c_int = 2;

pub const S_SIGTARABT: c_int = 1;

pub const S_MSTDETPARERR: c_int = 0;

pub const A_PCIX_INT_CAUSE: c_uint = 0x84;
pub const A_PCIX_CFG: c_uint = 0x88;
pub const S_DMASTOPEN: c_int = 19;

pub const S_CLIDECEN: c_int = 18;

pub const A_PCIX_MODE: c_uint = 0x8c;
pub const S_PCLKRANGE: c_int = 6;
pub const M_PCLKRANGE: c_uint = 0x3;

pub const S_PCIXINITPAT: c_int = 2;
pub const M_PCIXINITPAT: c_uint = 0xf;

pub const S_64BIT: c_int = 0;

pub const A_PCIE_INT_ENABLE: c_uint = 0x80;
pub const S_BISTERR: c_int = 15;
pub const M_BISTERR: c_uint = 0xff;

pub const S_TXPARERR: c_int = 18;

pub const S_RXPARERR: c_int = 17;

pub const S_RETRYLUTPARERR: c_int = 16;

pub const S_RETRYBUFPARERR: c_int = 15;

pub const S_PCIE_MSIXPARERR: c_int = 12;
pub const M_PCIE_MSIXPARERR: c_uint = 0x7;

pub const S_PCIE_CFPARERR: c_int = 11;

pub const S_PCIE_RFPARERR: c_int = 10;

pub const S_PCIE_WFPARERR: c_int = 9;

pub const S_PCIE_PIOPARERR: c_int = 8;

pub const S_UNXSPLCPLERRC: c_int = 7;

pub const S_UNXSPLCPLERRR: c_int = 6;

pub const S_PEXERR: c_int = 0;

pub const A_PCIE_INT_CAUSE: c_uint = 0x84;
pub const S_PCIE_DMASTOPEN: c_int = 24;

pub const A_PCIE_CFG: c_uint = 0x88;
pub const S_ENABLELINKDWNDRST: c_int = 21;

pub const S_ENABLELINKDOWNRST: c_int = 20;

pub const S_PCIE_CLIDECEN: c_int = 16;

pub const S_CRSTWRMMODE: c_int = 0;

pub const A_PCIE_MODE: c_uint = 0x8c;
pub const S_NUMFSTTRNSEQRX: c_int = 10;
pub const M_NUMFSTTRNSEQRX: c_uint = 0xff;

pub const A_PCIE_PEX_CTRL0: c_uint = 0x98;
pub const S_NUMFSTTRNSEQ: c_int = 22;
pub const M_NUMFSTTRNSEQ: c_uint = 0xff;

pub const S_REPLAYLMT: c_int = 2;
pub const M_REPLAYLMT: c_uint = 0xfffff;

pub const A_PCIE_PEX_CTRL1: c_uint = 0x9c;
pub const S_T3A_ACKLAT: c_int = 0;
pub const M_T3A_ACKLAT: c_uint = 0x7ff;

pub const S_ACKLAT: c_int = 0;
pub const M_ACKLAT: c_uint = 0x1fff;

pub const A_PCIE_PEX_ERR: c_uint = 0xa4;
pub const A_T3DBG_GPIO_EN: c_uint = 0xd0;
pub const S_GPIO11_OEN: c_int = 27;

pub const S_GPIO10_OEN: c_int = 26;

pub const S_GPIO7_OEN: c_int = 23;

pub const S_GPIO6_OEN: c_int = 22;

pub const S_GPIO5_OEN: c_int = 21;

pub const S_GPIO4_OEN: c_int = 20;

pub const S_GPIO2_OEN: c_int = 18;

pub const S_GPIO1_OEN: c_int = 17;

pub const S_GPIO0_OEN: c_int = 16;

pub const S_GPIO10_OUT_VAL: c_int = 10;

pub const S_GPIO7_OUT_VAL: c_int = 7;

pub const S_GPIO6_OUT_VAL: c_int = 6;

pub const S_GPIO5_OUT_VAL: c_int = 5;

pub const S_GPIO4_OUT_VAL: c_int = 4;

pub const S_GPIO2_OUT_VAL: c_int = 2;

pub const S_GPIO1_OUT_VAL: c_int = 1;

pub const S_GPIO0_OUT_VAL: c_int = 0;

pub const A_T3DBG_INT_ENABLE: c_uint = 0xd8;
pub const S_GPIO11: c_int = 11;

pub const S_GPIO10: c_int = 10;

pub const S_GPIO9: c_int = 9;

pub const S_GPIO7: c_int = 7;

pub const S_GPIO6: c_int = 6;

pub const S_GPIO5: c_int = 5;

pub const S_GPIO4: c_int = 4;

pub const S_GPIO3: c_int = 3;

pub const S_GPIO2: c_int = 2;

pub const S_GPIO1: c_int = 1;

pub const S_GPIO0: c_int = 0;

pub const A_T3DBG_INT_CAUSE: c_uint = 0xdc;
pub const A_T3DBG_GPIO_ACT_LOW: c_uint = 0xf0;
pub const MC7_PMRX_BASE_ADDR: c_uint = 0x100;
pub const A_MC7_CFG: c_uint = 0x100;
pub const S_IFEN: c_int = 13;

pub const S_TERM150: c_int = 11;

pub const S_SLOW: c_int = 10;

pub const S_WIDTH: c_int = 8;
pub const M_WIDTH: c_uint = 0x3;

pub const S_BKS: c_int = 6;

pub const S_ORG: c_int = 5;

pub const S_DEN: c_int = 2;
pub const M_DEN: c_uint = 0x7;

pub const S_RDY: c_int = 1;

pub const S_CLKEN: c_int = 0;

pub const A_MC7_MODE: c_uint = 0x104;
pub const S_BUSY: c_int = 31;

pub const A_MC7_EXT_MODE1: c_uint = 0x108;
pub const A_MC7_EXT_MODE2: c_uint = 0x10c;
pub const A_MC7_EXT_MODE3: c_uint = 0x110;
pub const A_MC7_PRE: c_uint = 0x114;
pub const A_MC7_REF: c_uint = 0x118;
pub const S_PREREFDIV: c_int = 1;
pub const M_PREREFDIV: c_uint = 0x3fff;

pub const S_PERREFEN: c_int = 0;

pub const A_MC7_DLL: c_uint = 0x11c;
pub const S_DLLENB: c_int = 1;

pub const S_DLLRST: c_int = 0;

pub const A_MC7_PARM: c_uint = 0x120;
pub const S_ACTTOPREDLY: c_int = 26;
pub const M_ACTTOPREDLY: c_uint = 0xf;

pub const S_ACTTORDWRDLY: c_int = 23;
pub const M_ACTTORDWRDLY: c_uint = 0x7;

pub const S_PRECYC: c_int = 20;
pub const M_PRECYC: c_uint = 0x7;

pub const S_REFCYC: c_int = 13;
pub const M_REFCYC: c_uint = 0x7f;

pub const S_BKCYC: c_int = 8;
pub const M_BKCYC: c_uint = 0x1f;

pub const S_WRTORDDLY: c_int = 4;
pub const M_WRTORDDLY: c_uint = 0xf;

pub const S_RDTOWRDLY: c_int = 0;
pub const M_RDTOWRDLY: c_uint = 0xf;

pub const A_MC7_CAL: c_uint = 0x128;
pub const S_CAL_FAULT: c_int = 30;

pub const S_SGL_CAL_EN: c_int = 20;

pub const A_MC7_ERR_ADDR: c_uint = 0x12c;
pub const A_MC7_ECC: c_uint = 0x130;
pub const S_ECCCHKEN: c_int = 1;

pub const S_ECCGENEN: c_int = 0;

pub const A_MC7_CE_ADDR: c_uint = 0x134;
pub const A_MC7_CE_DATA0: c_uint = 0x138;
pub const A_MC7_CE_DATA1: c_uint = 0x13c;
pub const A_MC7_CE_DATA2: c_uint = 0x140;
pub const S_DATA: c_int = 0;
pub const M_DATA: c_uint = 0xff;

pub const A_MC7_UE_ADDR: c_uint = 0x144;
pub const A_MC7_UE_DATA0: c_uint = 0x148;
pub const A_MC7_UE_DATA1: c_uint = 0x14c;
pub const A_MC7_UE_DATA2: c_uint = 0x150;
pub const A_MC7_BD_ADDR: c_uint = 0x154;
pub const S_ADDR: c_int = 3;
pub const M_ADDR: c_uint = 0x1fffffff;
pub const A_MC7_BD_DATA0: c_uint = 0x158;
pub const A_MC7_BD_DATA1: c_uint = 0x15c;
pub const A_MC7_BD_OP: c_uint = 0x164;
pub const S_OP: c_int = 0;

pub const A_MC7_BIST_ADDR_BEG: c_uint = 0x168;
pub const A_MC7_BIST_ADDR_END: c_uint = 0x16c;
pub const A_MC7_BIST_DATA: c_uint = 0x170;
pub const A_MC7_BIST_OP: c_uint = 0x174;
pub const S_CONT: c_int = 3;

pub const A_MC7_INT_ENABLE: c_uint = 0x178;
pub const S_AE: c_int = 17;

pub const S_PE: c_int = 2;
pub const M_PE: c_uint = 0x7fff;

pub const S_UE: c_int = 1;

pub const S_CE: c_int = 0;

pub const A_MC7_INT_CAUSE: c_uint = 0x17c;
pub const MC7_PMTX_BASE_ADDR: c_uint = 0x180;
pub const MC7_CM_BASE_ADDR: c_uint = 0x200;
pub const A_CIM_BOOT_CFG: c_uint = 0x280;
pub const S_BOOTADDR: c_int = 2;
pub const M_BOOTADDR: c_uint = 0x3fffffff;

pub const A_CIM_SDRAM_BASE_ADDR: c_uint = 0x28c;
pub const A_CIM_SDRAM_ADDR_SIZE: c_uint = 0x290;
pub const A_CIM_HOST_INT_ENABLE: c_uint = 0x298;
pub const S_DTAGPARERR: c_int = 28;

pub const S_ITAGPARERR: c_int = 27;

pub const S_IBQTPPARERR: c_int = 26;

pub const S_IBQULPPARERR: c_int = 25;

pub const S_IBQSGEHIPARERR: c_int = 24;

pub const S_IBQSGELOPARERR: c_int = 23;

pub const S_OBQULPLOPARERR: c_int = 22;

pub const S_OBQULPHIPARERR: c_int = 21;

pub const S_OBQSGEPARERR: c_int = 20;

pub const S_DCACHEPARERR: c_int = 19;

pub const S_ICACHEPARERR: c_int = 18;

pub const S_DRAMPARERR: c_int = 17;

pub const A_CIM_HOST_INT_CAUSE: c_uint = 0x29c;
pub const S_BLKWRPLINT: c_int = 12;

pub const S_BLKRDPLINT: c_int = 11;

pub const S_BLKWRCTLINT: c_int = 10;

pub const S_BLKRDCTLINT: c_int = 9;

pub const S_BLKWRFLASHINT: c_int = 8;

pub const S_BLKRDFLASHINT: c_int = 7;

pub const S_SGLWRFLASHINT: c_int = 6;

pub const S_WRBLKFLASHINT: c_int = 5;

pub const S_BLKWRBOOTINT: c_int = 4;

pub const S_FLASHRANGEINT: c_int = 2;

pub const S_SDRAMRANGEINT: c_int = 1;

pub const S_RSVDSPACEINT: c_int = 0;

pub const A_CIM_HOST_ACC_CTRL: c_uint = 0x2b0;
pub const S_HOSTBUSY: c_int = 17;

pub const A_CIM_HOST_ACC_DATA: c_uint = 0x2b4;
pub const A_CIM_IBQ_DBG_CFG: c_uint = 0x2c0;
pub const S_IBQDBGADDR: c_int = 16;
pub const M_IBQDBGADDR: c_uint = 0x1ff;

pub const S_IBQDBGQID: c_int = 3;
pub const M_IBQDBGQID: c_uint = 0x3;

pub const S_IBQDBGWR: c_int = 2;

pub const S_IBQDBGBUSY: c_int = 1;

pub const S_IBQDBGEN: c_int = 0;

pub const A_CIM_IBQ_DBG_DATA: c_uint = 0x2c8;
pub const A_TP_IN_CONFIG: c_uint = 0x300;
pub const S_RXFBARBPRIO: c_int = 25;

pub const S_TXFBARBPRIO: c_int = 24;

pub const S_NICMODE: c_int = 14;

pub const S_IPV6ENABLE: c_int = 15;

pub const A_TP_OUT_CONFIG: c_uint = 0x304;
pub const S_VLANEXTRACTIONENABLE: c_int = 12;
pub const A_TP_GLOBAL_CONFIG: c_uint = 0x308;
pub const S_TXPACINGENABLE: c_int = 24;

pub const S_PATHMTU: c_int = 15;

pub const S_IPCHECKSUMOFFLOAD: c_int = 13;

pub const S_UDPCHECKSUMOFFLOAD: c_int = 12;

pub const S_TCPCHECKSUMOFFLOAD: c_int = 11;

pub const S_IPTTL: c_int = 0;
pub const M_IPTTL: c_uint = 0xff;

pub const A_TP_CMM_MM_BASE: c_uint = 0x314;
pub const A_TP_CMM_TIMER_BASE: c_uint = 0x318;
pub const S_CMTIMERMAXNUM: c_int = 28;
pub const M_CMTIMERMAXNUM: c_uint = 0x3;

pub const A_TP_PMM_SIZE: c_uint = 0x31c;
pub const A_TP_PMM_TX_BASE: c_uint = 0x320;
pub const A_TP_PMM_RX_BASE: c_uint = 0x328;
pub const A_TP_PMM_RX_PAGE_SIZE: c_uint = 0x32c;
pub const A_TP_PMM_RX_MAX_PAGE: c_uint = 0x330;
pub const A_TP_PMM_TX_PAGE_SIZE: c_uint = 0x334;
pub const A_TP_PMM_TX_MAX_PAGE: c_uint = 0x338;
pub const A_TP_TCP_OPTIONS: c_uint = 0x340;
pub const S_MTUDEFAULT: c_int = 16;
pub const M_MTUDEFAULT: c_uint = 0xffff;

pub const S_MTUENABLE: c_int = 10;

pub const S_SACKRX: c_int = 8;

pub const S_SACKMODE: c_int = 4;
pub const M_SACKMODE: c_uint = 0x3;

pub const S_WINDOWSCALEMODE: c_int = 2;
pub const M_WINDOWSCALEMODE: c_uint = 0x3;

pub const S_TIMESTAMPSMODE: c_int = 0;
pub const M_TIMESTAMPSMODE: c_uint = 0x3;

pub const A_TP_DACK_CONFIG: c_uint = 0x344;
pub const S_AUTOSTATE3: c_int = 30;
pub const M_AUTOSTATE3: c_uint = 0x3;

pub const S_AUTOSTATE2: c_int = 28;
pub const M_AUTOSTATE2: c_uint = 0x3;

pub const S_AUTOSTATE1: c_int = 26;
pub const M_AUTOSTATE1: c_uint = 0x3;

pub const S_BYTETHRESHOLD: c_int = 5;
pub const M_BYTETHRESHOLD: c_uint = 0xfffff;

pub const S_MSSTHRESHOLD: c_int = 3;
pub const M_MSSTHRESHOLD: c_uint = 0x3;

pub const S_AUTOCAREFUL: c_int = 2;

pub const S_AUTOENABLE: c_int = 1;

pub const S_DACK_MODE: c_int = 0;

pub const A_TP_PC_CONFIG: c_uint = 0x348;
pub const S_TXTOSQUEUEMAPMODE: c_int = 26;

pub const S_ENABLEEPCMDAFULL: c_int = 23;

pub const S_MODULATEUNIONMODE: c_int = 22;

pub const S_TXDEFERENABLE: c_int = 20;

pub const S_RXCONGESTIONMODE: c_int = 19;

pub const S_HEARBEATDACK: c_int = 16;

pub const S_TXCONGESTIONMODE: c_int = 15;

pub const S_ENABLEOCSPIFULL: c_int = 30;

pub const S_LOCKTID: c_int = 28;

pub const S_TABLELATENCYDELTA: c_int = 0;
pub const M_TABLELATENCYDELTA: c_uint = 0xf;

pub const A_TP_PC_CONFIG2: c_uint = 0x34c;
pub const S_DISBLEDAPARBIT0: c_int = 15;

pub const S_ENABLEARPMISS: c_int = 13;

pub const S_ENABLENONOFDTNLSYN: c_int = 12;

pub const S_ENABLEIPV6RSS: c_int = 11;

pub const S_CHDRAFULL: c_int = 4;

pub const A_TP_TCP_BACKOFF_REG0: c_uint = 0x350;
pub const A_TP_TCP_BACKOFF_REG1: c_uint = 0x354;
pub const A_TP_TCP_BACKOFF_REG2: c_uint = 0x358;
pub const A_TP_TCP_BACKOFF_REG3: c_uint = 0x35c;
pub const A_TP_PARA_REG2: c_uint = 0x368;
pub const S_MAXRXDATA: c_int = 16;
pub const M_MAXRXDATA: c_uint = 0xffff;

pub const S_RXCOALESCESIZE: c_int = 0;
pub const M_RXCOALESCESIZE: c_uint = 0xffff;

pub const A_TP_PARA_REG3: c_uint = 0x36c;
pub const S_TXDATAACKIDX: c_int = 16;
pub const M_TXDATAACKIDX: c_uint = 0xf;

pub const S_TXPACEAUTOSTRICT: c_int = 10;

pub const S_TXPACEFIXED: c_int = 9;

pub const S_TXPACEAUTO: c_int = 8;

pub const S_RXCOALESCEENABLE: c_int = 1;

pub const S_RXCOALESCEPSHEN: c_int = 0;

pub const A_TP_PARA_REG4: c_uint = 0x370;
pub const A_TP_PARA_REG5: c_uint = 0x374;
pub const S_RXDDPOFFINIT: c_int = 3;

pub const A_TP_PARA_REG6: c_uint = 0x378;
pub const S_T3A_ENABLEESND: c_int = 13;

pub const S_ENABLEESND: c_int = 11;

pub const A_TP_PARA_REG7: c_uint = 0x37c;
pub const S_PMMAXXFERLEN1: c_int = 16;
pub const M_PMMAXXFERLEN1: c_uint = 0xffff;

pub const S_PMMAXXFERLEN0: c_int = 0;
pub const M_PMMAXXFERLEN0: c_uint = 0xffff;

pub const A_TP_TIMER_RESOLUTION: c_uint = 0x390;
pub const S_TIMERRESOLUTION: c_int = 16;
pub const M_TIMERRESOLUTION: c_uint = 0xff;

pub const S_TIMESTAMPRESOLUTION: c_int = 8;
pub const M_TIMESTAMPRESOLUTION: c_uint = 0xff;

pub const S_DELAYEDACKRESOLUTION: c_int = 0;
pub const M_DELAYEDACKRESOLUTION: c_uint = 0xff;

pub const A_TP_MSL: c_uint = 0x394;
pub const A_TP_RXT_MIN: c_uint = 0x398;
pub const A_TP_RXT_MAX: c_uint = 0x39c;
pub const A_TP_PERS_MIN: c_uint = 0x3a0;
pub const A_TP_PERS_MAX: c_uint = 0x3a4;
pub const A_TP_KEEP_IDLE: c_uint = 0x3a8;
pub const A_TP_KEEP_INTVL: c_uint = 0x3ac;
pub const A_TP_INIT_SRTT: c_uint = 0x3b0;
pub const A_TP_DACK_TIMER: c_uint = 0x3b4;
pub const A_TP_FINWAIT2_TIMER: c_uint = 0x3b8;
pub const A_TP_SHIFT_CNT: c_uint = 0x3c0;
pub const S_SYNSHIFTMAX: c_int = 24;
pub const M_SYNSHIFTMAX: c_uint = 0xff;

pub const S_RXTSHIFTMAXR1: c_int = 20;
pub const M_RXTSHIFTMAXR1: c_uint = 0xf;

pub const S_RXTSHIFTMAXR2: c_int = 16;
pub const M_RXTSHIFTMAXR2: c_uint = 0xf;

pub const S_PERSHIFTBACKOFFMAX: c_int = 12;
pub const M_PERSHIFTBACKOFFMAX: c_uint = 0xf;

pub const S_PERSHIFTMAX: c_int = 8;
pub const M_PERSHIFTMAX: c_uint = 0xf;

pub const S_KEEPALIVEMAX: c_int = 0;
pub const M_KEEPALIVEMAX: c_uint = 0xff;

pub const A_TP_MTU_PORT_TABLE: c_uint = 0x3d0;
pub const A_TP_CCTRL_TABLE: c_uint = 0x3dc;
pub const A_TP_MTU_TABLE: c_uint = 0x3e4;
pub const A_TP_RSS_MAP_TABLE: c_uint = 0x3e8;
pub const A_TP_RSS_LKP_TABLE: c_uint = 0x3ec;
pub const A_TP_RSS_CONFIG: c_uint = 0x3f0;
pub const S_TNL4TUPEN: c_int = 29;

pub const S_TNL2TUPEN: c_int = 28;

pub const S_TNLPRTEN: c_int = 26;

pub const S_TNLMAPEN: c_int = 25;

pub const S_TNLLKPEN: c_int = 24;

pub const S_RRCPLMAPEN: c_int = 7;

pub const S_RRCPLCPUSIZE: c_int = 4;
pub const M_RRCPLCPUSIZE: c_uint = 0x7;

pub const S_RQFEEDBACKENABLE: c_int = 3;

pub const S_HASHTOEPLITZ: c_int = 2;

pub const S_DISABLE: c_int = 0;
pub const A_TP_TM_PIO_ADDR: c_uint = 0x418;
pub const A_TP_TM_PIO_DATA: c_uint = 0x41c;
pub const A_TP_TX_MOD_QUE_TABLE: c_uint = 0x420;
pub const A_TP_TX_RESOURCE_LIMIT: c_uint = 0x424;
pub const A_TP_TX_MOD_QUEUE_REQ_MAP: c_uint = 0x428;
pub const S_TX_MOD_QUEUE_REQ_MAP: c_int = 0;
pub const M_TX_MOD_QUEUE_REQ_MAP: c_uint = 0xff;

pub const A_TP_TX_MOD_QUEUE_WEIGHT1: c_uint = 0x42c;
pub const A_TP_TX_MOD_QUEUE_WEIGHT0: c_uint = 0x430;
pub const A_TP_MOD_CHANNEL_WEIGHT: c_uint = 0x434;
pub const A_TP_MOD_RATE_LIMIT: c_uint = 0x438;
pub const A_TP_PIO_ADDR: c_uint = 0x440;
pub const A_TP_PIO_DATA: c_uint = 0x444;
pub const A_TP_RESET: c_uint = 0x44c;
pub const S_FLSTINITENABLE: c_int = 1;

pub const S_TPRESET: c_int = 0;

pub const A_TP_CMM_MM_RX_FLST_BASE: c_uint = 0x460;
pub const A_TP_CMM_MM_TX_FLST_BASE: c_uint = 0x464;
pub const A_TP_CMM_MM_PS_FLST_BASE: c_uint = 0x468;
pub const A_TP_MIB_INDEX: c_uint = 0x450;
pub const A_TP_MIB_RDATA: c_uint = 0x454;
pub const A_TP_CMM_MM_MAX_PSTRUCT: c_uint = 0x46c;
pub const A_TP_INT_ENABLE: c_uint = 0x470;
pub const S_FLMTXFLSTEMPTY: c_int = 30;

pub const S_FLMRXFLSTEMPTY: c_int = 29;

pub const S_ARPLUTPERR: c_int = 26;

pub const S_CMCACHEPERR: c_int = 24;

pub const A_TP_INT_CAUSE: c_uint = 0x474;
pub const A_TP_TX_MOD_Q1_Q0_RATE_LIMIT: c_uint = 0x8;
pub const A_TP_TX_DROP_CFG_CH0: c_uint = 0x12b;
pub const A_TP_TX_DROP_MODE: c_uint = 0x12f;
pub const A_TP_EGRESS_CONFIG: c_uint = 0x145;
pub const S_REWRITEFORCETOSIZE: c_int = 0;

pub const A_TP_TX_TRC_KEY0: c_uint = 0x20;
pub const A_TP_RX_TRC_KEY0: c_uint = 0x120;
pub const A_TP_TX_DROP_CNT_CH0: c_uint = 0x12d;
pub const S_TXDROPCNTCH0RCVD: c_int = 0;
pub const M_TXDROPCNTCH0RCVD: c_uint = 0xffff;

pub const A_TP_PROXY_FLOW_CNTL: c_uint = 0x4b0;
pub const A_TP_EMBED_OP_FIELD0: c_uint = 0x4e8;
pub const A_TP_EMBED_OP_FIELD1: c_uint = 0x4ec;
pub const A_TP_EMBED_OP_FIELD2: c_uint = 0x4f0;
pub const A_TP_EMBED_OP_FIELD3: c_uint = 0x4f4;
pub const A_TP_EMBED_OP_FIELD4: c_uint = 0x4f8;
pub const A_TP_EMBED_OP_FIELD5: c_uint = 0x4fc;
pub const A_ULPRX_CTL: c_uint = 0x500;
pub const S_ROUND_ROBIN: c_int = 4;

pub const A_ULPRX_INT_ENABLE: c_uint = 0x504;
pub const S_DATASELFRAMEERR0: c_int = 7;

pub const S_DATASELFRAMEERR1: c_int = 6;

pub const S_PCMDMUXPERR: c_int = 5;

pub const S_ARBFPERR: c_int = 4;

pub const S_ARBPF0PERR: c_int = 3;

pub const S_ARBPF1PERR: c_int = 2;

pub const S_PARERRPCMD: c_int = 1;

pub const S_PARERRDATA: c_int = 0;

pub const A_ULPRX_INT_CAUSE: c_uint = 0x508;
pub const A_ULPRX_ISCSI_LLIMIT: c_uint = 0x50c;
pub const A_ULPRX_ISCSI_ULIMIT: c_uint = 0x510;
pub const A_ULPRX_ISCSI_TAGMASK: c_uint = 0x514;
pub const A_ULPRX_ISCSI_PSZ: c_uint = 0x518;
pub const A_ULPRX_TDDP_LLIMIT: c_uint = 0x51c;
pub const A_ULPRX_TDDP_ULIMIT: c_uint = 0x520;
pub const A_ULPRX_TDDP_PSZ: c_uint = 0x528;
pub const S_HPZ0: c_int = 0;
pub const M_HPZ0: c_uint = 0xf;

pub const A_ULPRX_STAG_LLIMIT: c_uint = 0x52c;
pub const A_ULPRX_STAG_ULIMIT: c_uint = 0x530;
pub const A_ULPRX_RQ_LLIMIT: c_uint = 0x534;
pub const A_ULPRX_RQ_ULIMIT: c_uint = 0x538;
pub const A_ULPRX_PBL_LLIMIT: c_uint = 0x53c;
pub const A_ULPRX_PBL_ULIMIT: c_uint = 0x540;
pub const A_ULPRX_TDDP_TAGMASK: c_uint = 0x524;
pub const A_ULPTX_CONFIG: c_uint = 0x580;
pub const S_CFG_CQE_SOP_MASK: c_int = 1;

pub const S_CFG_RR_ARB: c_int = 0;

pub const A_ULPTX_INT_ENABLE: c_uint = 0x584;
pub const S_PBL_BOUND_ERR_CH1: c_int = 1;

pub const S_PBL_BOUND_ERR_CH0: c_int = 0;

pub const A_ULPTX_INT_CAUSE: c_uint = 0x588;
pub const A_ULPTX_TPT_LLIMIT: c_uint = 0x58c;
pub const A_ULPTX_TPT_ULIMIT: c_uint = 0x590;
pub const A_ULPTX_PBL_LLIMIT: c_uint = 0x594;
pub const A_ULPTX_PBL_ULIMIT: c_uint = 0x598;
pub const A_ULPTX_DMA_WEIGHT: c_uint = 0x5ac;
pub const S_D1_WEIGHT: c_int = 16;
pub const M_D1_WEIGHT: c_uint = 0xffff;

pub const S_D0_WEIGHT: c_int = 0;
pub const M_D0_WEIGHT: c_uint = 0xffff;

pub const A_PM1_RX_CFG: c_uint = 0x5c0;
pub const A_PM1_RX_MODE: c_uint = 0x5c4;
pub const A_PM1_RX_INT_ENABLE: c_uint = 0x5d8;
pub const S_ZERO_E_CMD_ERROR: c_int = 18;

pub const S_IESPI0_FIFO2X_RX_FRAMING_ERROR: c_int = 17;

pub const S_IESPI1_FIFO2X_RX_FRAMING_ERROR: c_int = 16;

pub const S_IESPI0_RX_FRAMING_ERROR: c_int = 15;

pub const S_IESPI1_RX_FRAMING_ERROR: c_int = 14;

pub const S_IESPI0_TX_FRAMING_ERROR: c_int = 13;

pub const S_IESPI1_TX_FRAMING_ERROR: c_int = 12;

pub const S_OCSPI0_RX_FRAMING_ERROR: c_int = 11;

pub const S_OCSPI1_RX_FRAMING_ERROR: c_int = 10;

pub const S_OCSPI0_TX_FRAMING_ERROR: c_int = 9;

pub const S_OCSPI1_TX_FRAMING_ERROR: c_int = 8;

pub const S_OCSPI0_OFIFO2X_TX_FRAMING_ERROR: c_int = 7;

pub const S_OCSPI1_OFIFO2X_TX_FRAMING_ERROR: c_int = 6;

pub const S_IESPI_PAR_ERROR: c_int = 3;
pub const M_IESPI_PAR_ERROR: c_uint = 0x7;

pub const S_OCSPI_PAR_ERROR: c_int = 0;
pub const M_OCSPI_PAR_ERROR: c_uint = 0x7;

pub const A_PM1_RX_INT_CAUSE: c_uint = 0x5dc;
pub const A_PM1_TX_CFG: c_uint = 0x5e0;
pub const A_PM1_TX_MODE: c_uint = 0x5e4;
pub const A_PM1_TX_INT_ENABLE: c_uint = 0x5f8;
pub const S_ZERO_C_CMD_ERROR: c_int = 18;

pub const S_ICSPI0_FIFO2X_RX_FRAMING_ERROR: c_int = 17;

pub const S_ICSPI1_FIFO2X_RX_FRAMING_ERROR: c_int = 16;

pub const S_ICSPI0_RX_FRAMING_ERROR: c_int = 15;

pub const S_ICSPI1_RX_FRAMING_ERROR: c_int = 14;

pub const S_ICSPI0_TX_FRAMING_ERROR: c_int = 13;

pub const S_ICSPI1_TX_FRAMING_ERROR: c_int = 12;

pub const S_OESPI0_RX_FRAMING_ERROR: c_int = 11;

pub const S_OESPI1_RX_FRAMING_ERROR: c_int = 10;

pub const S_OESPI0_TX_FRAMING_ERROR: c_int = 9;

pub const S_OESPI1_TX_FRAMING_ERROR: c_int = 8;

pub const S_OESPI0_OFIFO2X_TX_FRAMING_ERROR: c_int = 7;

pub const S_OESPI1_OFIFO2X_TX_FRAMING_ERROR: c_int = 6;

pub const S_ICSPI_PAR_ERROR: c_int = 3;
pub const M_ICSPI_PAR_ERROR: c_uint = 0x7;

pub const S_OESPI_PAR_ERROR: c_int = 0;
pub const M_OESPI_PAR_ERROR: c_uint = 0x7;

pub const A_PM1_TX_INT_CAUSE: c_uint = 0x5fc;
pub const A_MPS_CFG: c_uint = 0x600;
pub const S_TPRXPORTEN: c_int = 4;

pub const S_TPTXPORT1EN: c_int = 3;

pub const S_TPTXPORT0EN: c_int = 2;

pub const S_PORT1ACTIVE: c_int = 1;

pub const S_PORT0ACTIVE: c_int = 0;

pub const S_ENFORCEPKT: c_int = 11;

pub const A_MPS_INT_ENABLE: c_uint = 0x61c;
pub const S_MCAPARERRENB: c_int = 6;
pub const M_MCAPARERRENB: c_uint = 0x7;

pub const S_RXTPPARERRENB: c_int = 4;
pub const M_RXTPPARERRENB: c_uint = 0x3;

pub const S_TX1TPPARERRENB: c_int = 2;
pub const M_TX1TPPARERRENB: c_uint = 0x3;

pub const S_TX0TPPARERRENB: c_int = 0;
pub const M_TX0TPPARERRENB: c_uint = 0x3;

pub const A_MPS_INT_CAUSE: c_uint = 0x620;
pub const S_MCAPARERR: c_int = 6;
pub const M_MCAPARERR: c_uint = 0x7;

pub const S_RXTPPARERR: c_int = 4;
pub const M_RXTPPARERR: c_uint = 0x3;

pub const S_TX1TPPARERR: c_int = 2;
pub const M_TX1TPPARERR: c_uint = 0x3;

pub const S_TX0TPPARERR: c_int = 0;
pub const M_TX0TPPARERR: c_uint = 0x3;

pub const A_CPL_SWITCH_CNTRL: c_uint = 0x640;
pub const A_CPL_INTR_ENABLE: c_uint = 0x650;
pub const S_CIM_OP_MAP_PERR: c_int = 5;

pub const S_CIM_OVFL_ERROR: c_int = 4;

pub const S_TP_FRAMING_ERROR: c_int = 3;

pub const S_SGE_FRAMING_ERROR: c_int = 2;

pub const S_CIM_FRAMING_ERROR: c_int = 1;

pub const S_ZERO_SWITCH_ERROR: c_int = 0;

pub const A_CPL_INTR_CAUSE: c_uint = 0x654;
pub const A_CPL_MAP_TBL_DATA: c_uint = 0x65c;
pub const A_SMB_GLOBAL_TIME_CFG: c_uint = 0x660;
pub const A_I2C_CFG: c_uint = 0x6a0;
pub const S_I2C_CLKDIV: c_int = 0;
pub const M_I2C_CLKDIV: c_uint = 0xfff;

pub const A_MI1_CFG: c_uint = 0x6b0;
pub const S_CLKDIV: c_int = 5;
pub const M_CLKDIV: c_uint = 0xff;

pub const S_ST: c_int = 3;
pub const M_ST: c_uint = 0x3;

pub const S_PREEN: c_int = 2;

pub const S_MDIINV: c_int = 1;

pub const S_MDIEN: c_int = 0;

pub const A_MI1_ADDR: c_uint = 0x6b4;
pub const S_PHYADDR: c_int = 5;
pub const M_PHYADDR: c_uint = 0x1f;

pub const S_REGADDR: c_int = 0;
pub const M_REGADDR: c_uint = 0x1f;

pub const A_MI1_DATA: c_uint = 0x6b8;
pub const A_MI1_OP: c_uint = 0x6bc;
pub const S_MDI_OP: c_int = 0;
pub const M_MDI_OP: c_uint = 0x3;

pub const A_SF_DATA: c_uint = 0x6d8;
pub const A_SF_OP: c_uint = 0x6dc;
pub const S_BYTECNT: c_int = 1;
pub const M_BYTECNT: c_uint = 0x3;

pub const A_PL_INT_ENABLE0: c_uint = 0x6e0;
pub const S_T3DBG: c_int = 23;

pub const S_XGMAC0_1: c_int = 20;

pub const S_XGMAC0_0: c_int = 19;

pub const S_MC5A: c_int = 18;

pub const S_CPL_SWITCH: c_int = 12;

pub const S_MPS0: c_int = 11;

pub const S_PM1_TX: c_int = 10;

pub const S_PM1_RX: c_int = 9;

pub const S_ULP2_TX: c_int = 8;

pub const S_ULP2_RX: c_int = 7;

pub const S_TP1: c_int = 6;

pub const S_CIM: c_int = 5;

pub const S_MC7_CM: c_int = 4;

pub const S_MC7_PMTX: c_int = 3;

pub const S_MC7_PMRX: c_int = 2;

pub const S_PCIM0: c_int = 1;

pub const S_SGE3: c_int = 0;

pub const A_PL_INT_CAUSE0: c_uint = 0x6e4;
pub const A_PL_RST: c_uint = 0x6f0;
pub const S_FATALPERREN: c_int = 4;

pub const S_CRSTWRM: c_int = 1;

pub const A_PL_REV: c_uint = 0x6f4;
pub const A_PL_CLI: c_uint = 0x6f8;
pub const A_MC5_DB_CONFIG: c_uint = 0x704;
pub const S_TMTYPEHI: c_int = 30;

pub const S_TMPARTSIZE: c_int = 28;
pub const M_TMPARTSIZE: c_uint = 0x3;

pub const S_TMTYPE: c_int = 26;
pub const M_TMTYPE: c_uint = 0x3;

pub const S_COMPEN: c_int = 17;

pub const S_PRTYEN: c_int = 6;

pub const S_MBUSEN: c_int = 5;

pub const S_DBGIEN: c_int = 4;

pub const S_TMRDY: c_int = 2;

pub const S_TMRST: c_int = 1;

pub const S_TMMODE: c_int = 0;

pub const A_MC5_DB_ROUTING_TABLE_INDEX: c_uint = 0x70c;
pub const A_MC5_DB_FILTER_TABLE: c_uint = 0x710;
pub const A_MC5_DB_SERVER_INDEX: c_uint = 0x714;
pub const A_MC5_DB_RSP_LATENCY: c_uint = 0x720;
pub const S_RDLAT: c_int = 16;
pub const M_RDLAT: c_uint = 0x1f;

pub const S_LRNLAT: c_int = 8;
pub const M_LRNLAT: c_uint = 0x1f;

pub const S_SRCHLAT: c_int = 0;
pub const M_SRCHLAT: c_uint = 0x1f;

pub const A_MC5_DB_PART_ID_INDEX: c_uint = 0x72c;
pub const A_MC5_DB_INT_ENABLE: c_uint = 0x740;
pub const S_DELACTEMPTY: c_int = 18;

pub const S_DISPQPARERR: c_int = 17;

pub const S_REQQPARERR: c_int = 16;

pub const S_UNKNOWNCMD: c_int = 15;

pub const S_NFASRCHFAIL: c_int = 8;

pub const S_ACTRGNFULL: c_int = 7;

pub const S_PARITYERR: c_int = 6;

pub const A_MC5_DB_INT_CAUSE: c_uint = 0x744;
pub const A_MC5_DB_DBGI_CONFIG: c_uint = 0x774;
pub const A_MC5_DB_DBGI_REQ_CMD: c_uint = 0x778;
pub const A_MC5_DB_DBGI_REQ_ADDR0: c_uint = 0x77c;
pub const A_MC5_DB_DBGI_REQ_ADDR1: c_uint = 0x780;
pub const A_MC5_DB_DBGI_REQ_ADDR2: c_uint = 0x784;
pub const A_MC5_DB_DBGI_REQ_DATA0: c_uint = 0x788;
pub const A_MC5_DB_DBGI_REQ_DATA1: c_uint = 0x78c;
pub const A_MC5_DB_DBGI_REQ_DATA2: c_uint = 0x790;
pub const A_MC5_DB_DBGI_RSP_STATUS: c_uint = 0x7b0;
pub const S_DBGIRSPVALID: c_int = 0;

pub const A_MC5_DB_DBGI_RSP_DATA0: c_uint = 0x7b4;
pub const A_MC5_DB_DBGI_RSP_DATA1: c_uint = 0x7b8;
pub const A_MC5_DB_DBGI_RSP_DATA2: c_uint = 0x7bc;
pub const A_MC5_DB_POPEN_DATA_WR_CMD: c_uint = 0x7cc;
pub const A_MC5_DB_POPEN_MASK_WR_CMD: c_uint = 0x7d0;
pub const A_MC5_DB_AOPEN_SRCH_CMD: c_uint = 0x7d4;
pub const A_MC5_DB_AOPEN_LRN_CMD: c_uint = 0x7d8;
pub const A_MC5_DB_SYN_SRCH_CMD: c_uint = 0x7dc;
pub const A_MC5_DB_SYN_LRN_CMD: c_uint = 0x7e0;
pub const A_MC5_DB_ACK_SRCH_CMD: c_uint = 0x7e4;
pub const A_MC5_DB_ACK_LRN_CMD: c_uint = 0x7e8;
pub const A_MC5_DB_ILOOKUP_CMD: c_uint = 0x7ec;
pub const A_MC5_DB_ELOOKUP_CMD: c_uint = 0x7f0;
pub const A_MC5_DB_DATA_WRITE_CMD: c_uint = 0x7f4;
pub const A_MC5_DB_DATA_READ_CMD: c_uint = 0x7f8;
pub const XGMAC0_0_BASE_ADDR: c_uint = 0x800;
pub const A_XGM_TX_CTRL: c_uint = 0x800;
pub const S_TXEN: c_int = 0;

pub const A_XGM_TX_CFG: c_uint = 0x804;
pub const S_TXPAUSEEN: c_int = 0;

pub const A_XGM_TX_PAUSE_QUANTA: c_uint = 0x808;
pub const A_XGM_RX_CTRL: c_uint = 0x80c;
pub const S_RXEN: c_int = 0;

pub const A_XGM_RX_CFG: c_uint = 0x810;
pub const S_DISPAUSEFRAMES: c_int = 9;

pub const S_EN1536BFRAMES: c_int = 8;

pub const S_ENJUMBO: c_int = 7;

pub const S_RMFCS: c_int = 6;

pub const S_ENHASHMCAST: c_int = 2;

pub const S_COPYALLFRAMES: c_int = 0;

pub const S_DISBCAST: c_int = 1;

pub const A_XGM_RX_HASH_LOW: c_uint = 0x814;
pub const A_XGM_RX_HASH_HIGH: c_uint = 0x818;
pub const A_XGM_RX_EXACT_MATCH_LOW_1: c_uint = 0x81c;
pub const A_XGM_RX_EXACT_MATCH_HIGH_1: c_uint = 0x820;
pub const A_XGM_RX_EXACT_MATCH_LOW_2: c_uint = 0x824;
pub const A_XGM_RX_EXACT_MATCH_LOW_3: c_uint = 0x82c;
pub const A_XGM_RX_EXACT_MATCH_LOW_4: c_uint = 0x834;
pub const A_XGM_RX_EXACT_MATCH_LOW_5: c_uint = 0x83c;
pub const A_XGM_RX_EXACT_MATCH_LOW_6: c_uint = 0x844;
pub const A_XGM_RX_EXACT_MATCH_LOW_7: c_uint = 0x84c;
pub const A_XGM_RX_EXACT_MATCH_LOW_8: c_uint = 0x854;
pub const A_XGM_INT_STATUS: c_uint = 0x86c;
pub const S_LINKFAULTCHANGE: c_int = 9;

pub const A_XGM_XGM_INT_ENABLE: c_uint = 0x874;
pub const A_XGM_XGM_INT_DISABLE: c_uint = 0x878;
pub const A_XGM_STAT_CTRL: c_uint = 0x880;
pub const S_CLRSTATS: c_int = 2;

pub const A_XGM_RXFIFO_CFG: c_uint = 0x884;
pub const S_RXFIFO_EMPTY: c_int = 31;

pub const S_RXFIFOPAUSEHWM: c_int = 17;
pub const M_RXFIFOPAUSEHWM: c_uint = 0xfff;

pub const S_RXFIFOPAUSELWM: c_int = 5;
pub const M_RXFIFOPAUSELWM: c_uint = 0xfff;

pub const S_RXSTRFRWRD: c_int = 1;

pub const S_DISERRFRAMES: c_int = 0;

pub const A_XGM_TXFIFO_CFG: c_uint = 0x888;
pub const S_UNDERUNFIX: c_int = 22;

pub const S_TXIPG: c_int = 13;
pub const M_TXIPG: c_uint = 0xff;

pub const S_TXFIFOTHRESH: c_int = 4;
pub const M_TXFIFOTHRESH: c_uint = 0x1ff;

pub const S_ENDROPPKT: c_int = 21;

pub const A_XGM_SERDES_CTRL: c_uint = 0x890;
pub const A_XGM_SERDES_CTRL0: c_uint = 0x8e0;
pub const S_SERDESRESET_: c_int = 24;

pub const S_RXENABLE: c_int = 4;

pub const S_TXENABLE: c_int = 3;

pub const A_XGM_PAUSE_TIMER: c_uint = 0x890;
pub const A_XGM_RGMII_IMP: c_uint = 0x89c;
pub const S_XGM_IMPSETUPDATE: c_int = 6;

pub const S_RGMIIIMPPD: c_int = 3;
pub const M_RGMIIIMPPD: c_uint = 0x7;

pub const S_RGMIIIMPPU: c_int = 0;
pub const M_RGMIIIMPPU: c_uint = 0x7;

pub const S_CALRESET: c_int = 8;

pub const S_CALUPDATE: c_int = 7;

pub const A_XGM_XAUI_IMP: c_uint = 0x8a0;
pub const S_CALBUSY: c_int = 31;

pub const S_XGM_CALFAULT: c_int = 29;

pub const S_CALIMP: c_int = 24;
pub const M_CALIMP: c_uint = 0x1f;

pub const S_XAUIIMP: c_int = 0;
pub const M_XAUIIMP: c_uint = 0x7;

pub const A_XGM_RX_MAX_PKT_SIZE: c_uint = 0x8a8;
pub const S_RXMAXFRAMERSIZE: c_int = 17;
pub const M_RXMAXFRAMERSIZE: c_uint = 0x3fff;

pub const S_RXENFRAMER: c_int = 14;

pub const S_RXMAXPKTSIZE: c_int = 0;
pub const M_RXMAXPKTSIZE: c_uint = 0x3fff;

pub const A_XGM_RESET_CTRL: c_uint = 0x8ac;
pub const S_XGMAC_STOP_EN: c_int = 4;

pub const S_XG2G_RESET_: c_int = 3;

pub const S_RGMII_RESET_: c_int = 2;

pub const S_PCS_RESET_: c_int = 1;

pub const S_MAC_RESET_: c_int = 0;

pub const A_XGM_PORT_CFG: c_uint = 0x8b8;
pub const S_CLKDIVRESET_: c_int = 3;

pub const S_PORTSPEED: c_int = 1;
pub const M_PORTSPEED: c_uint = 0x3;

pub const S_ENRGMII: c_int = 0;

pub const A_XGM_INT_ENABLE: c_uint = 0x8d4;
pub const S_TXFIFO_PRTY_ERR: c_int = 17;
pub const M_TXFIFO_PRTY_ERR: c_uint = 0x7;

pub const S_RXFIFO_PRTY_ERR: c_int = 14;
pub const M_RXFIFO_PRTY_ERR: c_uint = 0x7;

pub const S_TXFIFO_UNDERRUN: c_int = 13;

pub const S_RXFIFO_OVERFLOW: c_int = 12;

pub const S_SERDES_LOS: c_int = 4;
pub const M_SERDES_LOS: c_uint = 0xf;

pub const S_XAUIPCSCTCERR: c_int = 3;

pub const S_XAUIPCSALIGNCHANGE: c_int = 2;

pub const S_XGM_INT: c_int = 0;

pub const A_XGM_INT_CAUSE: c_uint = 0x8d8;
pub const A_XGM_XAUI_ACT_CTRL: c_uint = 0x8dc;
pub const S_TXACTENABLE: c_int = 1;

pub const S_RESET3: c_int = 23;

pub const S_RESET2: c_int = 22;

pub const S_RESET1: c_int = 21;

pub const S_RESET0: c_int = 20;

pub const S_PWRDN3: c_int = 19;

pub const S_PWRDN2: c_int = 18;

pub const S_PWRDN1: c_int = 17;

pub const S_PWRDN0: c_int = 16;

pub const S_RESETPLL23: c_int = 15;

pub const S_RESETPLL01: c_int = 14;

pub const A_XGM_SERDES_STAT0: c_uint = 0x8f0;
pub const A_XGM_SERDES_STAT1: c_uint = 0x8f4;
pub const A_XGM_SERDES_STAT2: c_uint = 0x8f8;
pub const S_LOWSIG0: c_int = 0;

pub const A_XGM_SERDES_STAT3: c_uint = 0x8fc;
pub const A_XGM_STAT_TX_BYTE_LOW: c_uint = 0x900;
pub const A_XGM_STAT_TX_BYTE_HIGH: c_uint = 0x904;
pub const A_XGM_STAT_TX_FRAME_LOW: c_uint = 0x908;
pub const A_XGM_STAT_TX_FRAME_HIGH: c_uint = 0x90c;
pub const A_XGM_STAT_TX_BCAST: c_uint = 0x910;
pub const A_XGM_STAT_TX_MCAST: c_uint = 0x914;
pub const A_XGM_STAT_TX_PAUSE: c_uint = 0x918;
pub const A_XGM_STAT_TX_64B_FRAMES: c_uint = 0x91c;
pub const A_XGM_STAT_TX_65_127B_FRAMES: c_uint = 0x920;
pub const A_XGM_STAT_TX_128_255B_FRAMES: c_uint = 0x924;
pub const A_XGM_STAT_TX_256_511B_FRAMES: c_uint = 0x928;
pub const A_XGM_STAT_TX_512_1023B_FRAMES: c_uint = 0x92c;
pub const A_XGM_STAT_TX_1024_1518B_FRAMES: c_uint = 0x930;
pub const A_XGM_STAT_TX_1519_MAXB_FRAMES: c_uint = 0x934;
pub const A_XGM_STAT_TX_ERR_FRAMES: c_uint = 0x938;
pub const A_XGM_STAT_RX_BYTES_LOW: c_uint = 0x93c;
pub const A_XGM_STAT_RX_BYTES_HIGH: c_uint = 0x940;
pub const A_XGM_STAT_RX_FRAMES_LOW: c_uint = 0x944;
pub const A_XGM_STAT_RX_FRAMES_HIGH: c_uint = 0x948;
pub const A_XGM_STAT_RX_BCAST_FRAMES: c_uint = 0x94c;
pub const A_XGM_STAT_RX_MCAST_FRAMES: c_uint = 0x950;
pub const A_XGM_STAT_RX_PAUSE_FRAMES: c_uint = 0x954;
pub const A_XGM_STAT_RX_64B_FRAMES: c_uint = 0x958;
pub const A_XGM_STAT_RX_65_127B_FRAMES: c_uint = 0x95c;
pub const A_XGM_STAT_RX_128_255B_FRAMES: c_uint = 0x960;
pub const A_XGM_STAT_RX_256_511B_FRAMES: c_uint = 0x964;
pub const A_XGM_STAT_RX_512_1023B_FRAMES: c_uint = 0x968;
pub const A_XGM_STAT_RX_1024_1518B_FRAMES: c_uint = 0x96c;
pub const A_XGM_STAT_RX_1519_MAXB_FRAMES: c_uint = 0x970;
pub const A_XGM_STAT_RX_SHORT_FRAMES: c_uint = 0x974;
pub const A_XGM_STAT_RX_OVERSIZE_FRAMES: c_uint = 0x978;
pub const A_XGM_STAT_RX_JABBER_FRAMES: c_uint = 0x97c;
pub const A_XGM_STAT_RX_CRC_ERR_FRAMES: c_uint = 0x980;
pub const A_XGM_STAT_RX_LENGTH_ERR_FRAMES: c_uint = 0x984;
pub const A_XGM_STAT_RX_SYM_CODE_ERR_FRAMES: c_uint = 0x988;
pub const A_XGM_SERDES_STATUS0: c_uint = 0x98c;
pub const A_XGM_SERDES_STATUS1: c_uint = 0x990;
pub const S_CMULOCK: c_int = 31;

pub const A_XGM_RX_MAX_PKT_SIZE_ERR_CNT: c_uint = 0x9a4;
pub const A_XGM_TX_SPI4_SOP_EOP_CNT: c_uint = 0x9a8;
pub const S_TXSPI4SOPCNT: c_int = 16;
pub const M_TXSPI4SOPCNT: c_uint = 0xffff;

pub const A_XGM_RX_SPI4_SOP_EOP_CNT: c_uint = 0x9ac;
pub const XGMAC0_1_BASE_ADDR: c_uint = 0xa00;
