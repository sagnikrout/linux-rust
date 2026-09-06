//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/dwc2/hw.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
//
// hw.h - DesignWare HS OTG Controller hardware definitions
//
// Copyright 2004-2013 Synopsys, Inc.
//

pub const GOTGCTL_MULT_VALID_BC_SHIFT: c_int = 22;

pub const GAHBCFG_HBSTLEN_SHIFT: c_int = 1;
pub const GAHBCFG_HBSTLEN_SINGLE: c_int = 0;
pub const GAHBCFG_HBSTLEN_INCR: c_int = 1;
pub const GAHBCFG_HBSTLEN_INCR4: c_int = 3;
pub const GAHBCFG_HBSTLEN_INCR8: c_int = 5;
pub const GAHBCFG_HBSTLEN_INCR16: c_int = 7;

pub const GUSBCFG_USBTRDTIM_SHIFT: c_int = 10;

pub const GUSBCFG_TOUTCAL_SHIFT: c_int = 0;
pub const GUSBCFG_TOUTCAL_LIMIT: c_uint = 0x7;

pub const GRSTCTL_CLOCK_SWITH_TIMER_SHIFT: c_int = 11;
pub const GRSTCTL_CLOCK_SWITH_TIMER_VALUE_19: c_uint = 0x0;
pub const GRSTCTL_CLOCK_SWITH_TIMER_VALUE_15: c_uint = 0x1;
pub const GRSTCTL_CLOCK_SWITH_TIMER_VALUE_147: c_uint = 0x2;
pub const GRSTCTL_CLOCK_SWITH_TIMER_VALUE_50: c_uint = 0x3;
pub const GRSTCTL_CLOCK_SWITH_TIMER_VALUE_100: c_uint = 0x4;
pub const GRSTCTL_CLOCK_SWITH_TIMER_VALUE_125: c_uint = 0x5;
pub const GRSTCTL_CLOCK_SWITH_TIMER_VALUE_200: c_uint = 0x6;
pub const GRSTCTL_CLOCK_SWITH_TIMER_VALUE_DIS: c_uint = 0x7;

pub const GRSTCTL_TXFNUM_SHIFT: c_int = 6;
pub const GRSTCTL_TXFNUM_LIMIT: c_uint = 0x1f;

pub const GRXSTS_FN_SHIFT: c_int = 25;

pub const GRXSTS_PKTSTS_SHIFT: c_int = 17;
pub const GRXSTS_PKTSTS_GLOBALOUTNAK: c_int = 1;
pub const GRXSTS_PKTSTS_OUTRX: c_int = 2;
pub const GRXSTS_PKTSTS_HCHIN: c_int = 2;
pub const GRXSTS_PKTSTS_OUTDONE: c_int = 3;
pub const GRXSTS_PKTSTS_HCHIN_XFER_COMP: c_int = 3;
pub const GRXSTS_PKTSTS_SETUPDONE: c_int = 4;
pub const GRXSTS_PKTSTS_DATATOGGLEERR: c_int = 5;
pub const GRXSTS_PKTSTS_SETUPRX: c_int = 6;
pub const GRXSTS_PKTSTS_HCHHALTED: c_int = 7;

pub const GRXSTS_HCHNUM_SHIFT: c_int = 0;

pub const GRXSTS_DPID_SHIFT: c_int = 15;

pub const GRXSTS_BYTECNT_SHIFT: c_int = 4;

pub const GRXSTS_EPNUM_SHIFT: c_int = 0;

pub const GRXFSIZ_DEPTH_SHIFT: c_int = 0;

// Use FIFOSIZE_* constants to access this register

pub const GNPTXSTS_NP_TXQ_TOP_SHIFT: c_int = 24;

pub const GNPTXSTS_NP_TXQ_SPC_AVAIL_SHIFT: c_int = 16;

pub const GNPTXSTS_NP_TXF_SPC_AVAIL_SHIFT: c_int = 0;

pub const GI2CCTL_I2CDEVADDR_SHIFT: c_int = 26;

pub const GI2CCTL_ADDR_SHIFT: c_int = 16;

pub const GI2CCTL_REGADDR_SHIFT: c_int = 8;

pub const GI2CCTL_RWDATA_SHIFT: c_int = 0;

pub const GHWCFG2_DEV_TOKEN_Q_DEPTH_SHIFT: c_int = 26;

pub const GHWCFG2_HOST_PERIO_TX_Q_DEPTH_SHIFT: c_int = 24;

pub const GHWCFG2_NONPERIO_TX_Q_DEPTH_SHIFT: c_int = 22;

pub const GHWCFG2_NUM_HOST_CHAN_SHIFT: c_int = 14;

pub const GHWCFG2_NUM_DEV_EP_SHIFT: c_int = 10;

pub const GHWCFG2_FS_PHY_TYPE_SHIFT: c_int = 8;
pub const GHWCFG2_FS_PHY_TYPE_NOT_SUPPORTED: c_int = 0;
pub const GHWCFG2_FS_PHY_TYPE_DEDICATED: c_int = 1;
pub const GHWCFG2_FS_PHY_TYPE_SHARED_UTMI: c_int = 2;
pub const GHWCFG2_FS_PHY_TYPE_SHARED_ULPI: c_int = 3;

pub const GHWCFG2_HS_PHY_TYPE_SHIFT: c_int = 6;
pub const GHWCFG2_HS_PHY_TYPE_NOT_SUPPORTED: c_int = 0;
pub const GHWCFG2_HS_PHY_TYPE_UTMI: c_int = 1;
pub const GHWCFG2_HS_PHY_TYPE_ULPI: c_int = 2;
pub const GHWCFG2_HS_PHY_TYPE_UTMI_ULPI: c_int = 3;

pub const GHWCFG2_ARCHITECTURE_SHIFT: c_int = 3;
pub const GHWCFG2_SLAVE_ONLY_ARCH: c_int = 0;
pub const GHWCFG2_EXT_DMA_ARCH: c_int = 1;
pub const GHWCFG2_INT_DMA_ARCH: c_int = 2;

pub const GHWCFG2_OP_MODE_SHIFT: c_int = 0;
pub const GHWCFG2_OP_MODE_HNP_SRP_CAPABLE: c_int = 0;
pub const GHWCFG2_OP_MODE_SRP_ONLY_CAPABLE: c_int = 1;
pub const GHWCFG2_OP_MODE_NO_HNP_SRP_CAPABLE: c_int = 2;
pub const GHWCFG2_OP_MODE_SRP_CAPABLE_DEVICE: c_int = 3;
pub const GHWCFG2_OP_MODE_NO_SRP_CAPABLE_DEVICE: c_int = 4;
pub const GHWCFG2_OP_MODE_SRP_CAPABLE_HOST: c_int = 5;
pub const GHWCFG2_OP_MODE_NO_SRP_CAPABLE_HOST: c_int = 6;
pub const GHWCFG2_OP_MODE_UNDEFINED: c_int = 7;

pub const GHWCFG3_DFIFO_DEPTH_SHIFT: c_int = 16;

pub const GHWCFG3_PACKET_SIZE_CNTR_WIDTH_SHIFT: c_int = 4;

pub const GHWCFG3_XFER_SIZE_CNTR_WIDTH_SHIFT: c_int = 0;

pub const GHWCFG4_NUM_IN_EPS_SHIFT: c_int = 26;

pub const GHWCFG4_DED_FIFO_SHIFT: c_int = 25;

pub const GHWCFG4_NUM_DEV_MODE_CTRL_EP_SHIFT: c_int = 16;

pub const GHWCFG4_UTMI_PHY_DATA_WIDTH_SHIFT: c_int = 14;
pub const GHWCFG4_UTMI_PHY_DATA_WIDTH_8: c_int = 0;
pub const GHWCFG4_UTMI_PHY_DATA_WIDTH_16: c_int = 1;
pub const GHWCFG4_UTMI_PHY_DATA_WIDTH_8_OR_16: c_int = 2;

pub const GHWCFG4_NUM_DEV_PERIO_IN_EP_SHIFT: c_int = 0;

pub const GLPMCFG_LPM_RETRYCNT_STS_SHIFT: c_int = 25;

pub const GLPMCFG_RETRY_CNT_SHIFT: c_int = 21;

pub const GLPMCFG_LPM_CHNL_INDX_SHIFT: c_int = 17;

pub const GLPMCFG_COREL1RES_SHIFT: c_int = 13;

pub const GLPMCFG_HIRD_THRES_SHIFT: c_int = 8;

pub const GLPMCFG_HIRD_SHIFT: c_int = 2;

pub const GPWRDN_MULT_VAL_ID_BC_SHIFT: c_int = 24;

pub const GPWRDN_LINESTATE_SHIFT: c_int = 19;

pub const GDFIFOCFG_EPINFOBASE_SHIFT: c_int = 16;

pub const GDFIFOCFG_GDFIFOCFG_SHIFT: c_int = 0;

pub const ADPCTL_AR_SHIFT: c_int = 27;

pub const ADPCTL_RTIM_SHIFT: c_int = 6;

pub const ADPCTL_PRB_PER_SHIFT: c_int = 4;

pub const ADPCTL_PRB_DELTA_SHIFT: c_int = 2;

pub const ADPCTL_PRB_DSCHRG_SHIFT: c_int = 0;

pub const GREFCLK_REFCLKPER_SHIFT: c_int = 15;

pub const GREFCLK_SOF_CNT_WKUP_ALERT_SHIFT: c_int = 0;

// Use FIFOSIZE_* constants to access this register

// Use FIFOSIZE_* constants to access this register
// These apply to the GNPTXFSIZ, HPTXFSIZ and DPTXFSIZN registers

pub const FIFOSIZE_DEPTH_SHIFT: c_int = 16;

pub const FIFOSIZE_STARTADDR_SHIFT: c_int = 0;

// Device mode registers

pub const DCFG_EPMISCNT_SHIFT: c_int = 18;
pub const DCFG_EPMISCNT_LIMIT: c_uint = 0x1f;

pub const DCFG_PERFRINT_SHIFT: c_int = 11;
pub const DCFG_PERFRINT_LIMIT: c_uint = 0x3;

pub const DCFG_DEVADDR_SHIFT: c_int = 4;
pub const DCFG_DEVADDR_LIMIT: c_uint = 0x7f;

pub const DCFG_DEVSPD_SHIFT: c_int = 0;
pub const DCFG_DEVSPD_HS: c_int = 0;
pub const DCFG_DEVSPD_FS: c_int = 1;
pub const DCFG_DEVSPD_LS: c_int = 2;
pub const DCFG_DEVSPD_FS48: c_int = 3;

pub const DCTL_TSTCTL_SHIFT: c_int = 4;

pub const DSTS_SOFFN_SHIFT: c_int = 8;
pub const DSTS_SOFFN_LIMIT: c_uint = 0x3fff;

pub const DSTS_ENUMSPD_SHIFT: c_int = 1;
pub const DSTS_ENUMSPD_HS: c_int = 0;
pub const DSTS_ENUMSPD_FS: c_int = 1;
pub const DSTS_ENUMSPD_LS: c_int = 2;
pub const DSTS_ENUMSPD_FS48: c_int = 3;

pub const DAINT_OUTEP_SHIFT: c_int = 16;

// EP0 specialness:
// bits[29..28] - reserved (no SetD0PID, SetD1PID)
// bits[25..22] - should always be zero, this isn't a periodic endpoint
// bits[10..0]  - MPS setting different for EP0
//

pub const D0EPCTL_MPS_SHIFT: c_int = 0;
pub const D0EPCTL_MPS_64: c_int = 0;
pub const D0EPCTL_MPS_32: c_int = 1;
pub const D0EPCTL_MPS_16: c_int = 2;
pub const D0EPCTL_MPS_8: c_int = 3;

pub const DXEPCTL_TXFNUM_SHIFT: c_int = 22;
pub const DXEPCTL_TXFNUM_LIMIT: c_uint = 0xf;

pub const DXEPCTL_NEXTEP_SHIFT: c_int = 11;
pub const DXEPCTL_NEXTEP_LIMIT: c_uint = 0xf;

pub const DXEPCTL_MPS_SHIFT: c_int = 0;
pub const DXEPCTL_MPS_LIMIT: c_uint = 0x7ff;

pub const DIEPTSIZ0_PKTCNT_SHIFT: c_int = 19;
pub const DIEPTSIZ0_PKTCNT_LIMIT: c_uint = 0x3;

pub const DIEPTSIZ0_XFERSIZE_SHIFT: c_int = 0;
pub const DIEPTSIZ0_XFERSIZE_LIMIT: c_uint = 0x7f;

pub const DOEPTSIZ0_SUPCNT_SHIFT: c_int = 29;
pub const DOEPTSIZ0_SUPCNT_LIMIT: c_uint = 0x3;

pub const DOEPTSIZ0_XFERSIZE_SHIFT: c_int = 0;

pub const DXEPTSIZ_MC_SHIFT: c_int = 29;
pub const DXEPTSIZ_MC_LIMIT: c_uint = 0x3;

pub const DXEPTSIZ_PKTCNT_SHIFT: c_int = 19;
pub const DXEPTSIZ_PKTCNT_LIMIT: c_uint = 0x3ff;

pub const DXEPTSIZ_XFERSIZE_SHIFT: c_int = 0;
pub const DXEPTSIZ_XFERSIZE_LIMIT: c_uint = 0x7ffff;

pub const PCGCTL_P2HD_PRT_SPD_SHIFT: c_int = 29;

pub const PCGCTL_P2HD_DEV_ENUM_SPD_SHIFT: c_int = 27;

pub const PCGCTL_MAC_DEV_ADDR_SHIFT: c_int = 20;

pub const PCGCTL_MAX_XCVRSELECT_SHIFT: c_int = 17;

pub const PCGCTL_PRT_CLK_SEL_SHIFT: c_int = 14;

// Host Mode Registers

pub const HCFG_FRLISTEN_SHIFT: c_int = 24;

pub const FRLISTEN_8_SIZE: c_int = 8;

pub const FRLISTEN_16_SIZE: c_int = 16;

pub const FRLISTEN_32_SIZE: c_int = 32;

pub const FRLISTEN_64_SIZE: c_int = 64;

pub const HCFG_RESVALID_SHIFT: c_int = 8;

pub const HCFG_FSLSPCLKSEL_SHIFT: c_int = 0;
pub const HCFG_FSLSPCLKSEL_30_60_MHZ: c_int = 0;
pub const HCFG_FSLSPCLKSEL_48_MHZ: c_int = 1;
pub const HCFG_FSLSPCLKSEL_6_MHZ: c_int = 2;

pub const HFIR_FRINT_SHIFT: c_int = 0;

pub const HFNUM_FRREM_SHIFT: c_int = 16;

pub const HFNUM_FRNUM_SHIFT: c_int = 0;
pub const HFNUM_MAX_FRNUM: c_uint = 0x3fff;

pub const TXSTS_QTOP_CHNEP_SHIFT: c_int = 27;

pub const TXSTS_QTOP_TOKEN_SHIFT: c_int = 25;

pub const TXSTS_QSPCAVAIL_SHIFT: c_int = 16;

pub const TXSTS_FSPCAVAIL_SHIFT: c_int = 0;

pub const HPRT0_SPD_SHIFT: c_int = 17;
pub const HPRT0_SPD_HIGH_SPEED: c_int = 0;
pub const HPRT0_SPD_FULL_SPEED: c_int = 1;
pub const HPRT0_SPD_LOW_SPEED: c_int = 2;

pub const HPRT0_TSTCTL_SHIFT: c_int = 13;

pub const HPRT0_LNSTS_SHIFT: c_int = 10;

pub const HCCHAR_DEVADDR_SHIFT: c_int = 22;

pub const HCCHAR_MULTICNT_SHIFT: c_int = 20;

pub const HCCHAR_EPTYPE_SHIFT: c_int = 18;

pub const HCCHAR_EPNUM_SHIFT: c_int = 11;

pub const HCCHAR_MPS_SHIFT: c_int = 0;

pub const HCSPLT_XACTPOS_SHIFT: c_int = 14;
pub const HCSPLT_XACTPOS_MID: c_int = 0;
pub const HCSPLT_XACTPOS_END: c_int = 1;
pub const HCSPLT_XACTPOS_BEGIN: c_int = 2;
pub const HCSPLT_XACTPOS_ALL: c_int = 3;

pub const HCSPLT_HUBADDR_SHIFT: c_int = 7;

pub const HCSPLT_PRTADDR_SHIFT: c_int = 0;

pub const TSIZ_SC_MC_PID_SHIFT: c_int = 29;
pub const TSIZ_SC_MC_PID_DATA0: c_int = 0;
pub const TSIZ_SC_MC_PID_DATA2: c_int = 1;
pub const TSIZ_SC_MC_PID_DATA1: c_int = 2;
pub const TSIZ_SC_MC_PID_MDATA: c_int = 3;
pub const TSIZ_SC_MC_PID_SETUP: c_int = 3;

pub const TSIZ_PKTCNT_SHIFT: c_int = 19;

pub const TSIZ_NTD_SHIFT: c_int = 8;

pub const TSIZ_SCHINFO_SHIFT: c_int = 0;

pub const TSIZ_XFERSIZE_SHIFT: c_int = 0;

//
// struct dwc2_dma_desc - DMA descriptor structure,
// used for both host and gadget modes
//
// @status: DMA descriptor status quadlet
// @buf:    DMA descriptor data buffer pointer
//
// DMA Descriptor structure contains two quadlets:
// Status quadlet and Data buffer pointer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc2_dma_desc {
    pub status: u32,
    pub buf: u32,
    pub __packed: },
// Host Mode DMA descriptor status quadlet

pub const HOST_DMA_STS_SHIFT: c_int = 28;

pub const HOST_DMA_QTD_OFFSET_SHIFT: c_int = 17;

pub const HOST_DMA_ISOC_NBYTES_SHIFT: c_int = 0;

pub const HOST_DMA_NBYTES_SHIFT: c_int = 0;
pub const HOST_DMA_NBYTES_LIMIT: c_int = 131071;
// Device Mode DMA descriptor status quadlet

pub const DEV_DMA_BUFF_STS_SHIFT: c_int = 30;
pub const DEV_DMA_BUFF_STS_HREADY: c_int = 0;
pub const DEV_DMA_BUFF_STS_DMABUSY: c_int = 1;
pub const DEV_DMA_BUFF_STS_DMADONE: c_int = 2;
pub const DEV_DMA_BUFF_STS_HBUSY: c_int = 3;

pub const DEV_DMA_STS_SHIFT: c_int = 28;
pub const DEV_DMA_STS_SUCC: c_int = 0;
pub const DEV_DMA_STS_BUFF_FLUSH: c_int = 1;
pub const DEV_DMA_STS_BUFF_ERR: c_int = 3;

pub const DEV_DMA_ISOC_PID_SHIFT: c_int = 23;
pub const DEV_DMA_ISOC_PID_DATA0: c_int = 0;
pub const DEV_DMA_ISOC_PID_DATA2: c_int = 1;
pub const DEV_DMA_ISOC_PID_DATA1: c_int = 2;
pub const DEV_DMA_ISOC_PID_MDATA: c_int = 3;

pub const DEV_DMA_ISOC_FRNUM_SHIFT: c_int = 12;

pub const DEV_DMA_ISOC_TX_NBYTES_LIMIT: c_uint = 0xfff;

pub const DEV_DMA_ISOC_RX_NBYTES_LIMIT: c_uint = 0x7ff;
pub const DEV_DMA_ISOC_NBYTES_SHIFT: c_int = 0;

pub const DEV_DMA_NBYTES_SHIFT: c_int = 0;
pub const DEV_DMA_NBYTES_LIMIT: c_uint = 0xffff;
pub const MAX_DMA_DESC_NUM_GENERIC: c_int = 64;
pub const MAX_DMA_DESC_NUM_HS_ISOC: c_int = 256;
