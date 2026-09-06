//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/oss/oss_1_0_d.h
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


//
// Copyright (C) 2016 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
// AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
pub const ixCLIENT0_BM: c_uint = 0x0220;
pub const ixCLIENT0_CD0: c_uint = 0x0210;
pub const ixCLIENT0_CD1: c_uint = 0x0214;
pub const ixCLIENT0_CD2: c_uint = 0x0218;
pub const ixCLIENT0_CD3: c_uint = 0x021C;
pub const ixCLIENT0_CK0: c_uint = 0x0200;
pub const ixCLIENT0_CK1: c_uint = 0x0204;
pub const ixCLIENT0_CK2: c_uint = 0x0208;
pub const ixCLIENT0_CK3: c_uint = 0x020C;
pub const ixCLIENT0_K0: c_uint = 0x01F0;
pub const ixCLIENT0_K1: c_uint = 0x01F4;
pub const ixCLIENT0_K2: c_uint = 0x01F8;
pub const ixCLIENT0_K3: c_uint = 0x01FC;
pub const ixCLIENT0_OFFSET: c_uint = 0x0224;
pub const ixCLIENT0_OFFSET_HI: c_uint = 0x0290;
pub const ixCLIENT0_STATUS: c_uint = 0x0228;
pub const ixCLIENT1_BM: c_uint = 0x025C;
pub const ixCLIENT1_CD0: c_uint = 0x024C;
pub const ixCLIENT1_CD1: c_uint = 0x0250;
pub const ixCLIENT1_CD2: c_uint = 0x0254;
pub const ixCLIENT1_CD3: c_uint = 0x0258;
pub const ixCLIENT1_CK0: c_uint = 0x023C;
pub const ixCLIENT1_CK1: c_uint = 0x0240;
pub const ixCLIENT1_CK2: c_uint = 0x0244;
pub const ixCLIENT1_CK3: c_uint = 0x0248;
pub const ixCLIENT1_K0: c_uint = 0x022C;
pub const ixCLIENT1_K1: c_uint = 0x0230;
pub const ixCLIENT1_K2: c_uint = 0x0234;
pub const ixCLIENT1_K3: c_uint = 0x0238;
pub const ixCLIENT1_OFFSET: c_uint = 0x0260;
pub const ixCLIENT1_OFFSET_HI: c_uint = 0x0294;
pub const ixCLIENT1_PORT_STATUS: c_uint = 0x0264;
pub const ixCLIENT2_BM: c_uint = 0x01E4;
pub const ixCLIENT2_CD0: c_uint = 0x01D4;
pub const ixCLIENT2_CD1: c_uint = 0x01D8;
pub const ixCLIENT2_CD2: c_uint = 0x01DC;
pub const ixCLIENT2_CD3: c_uint = 0x01E0;
pub const ixCLIENT2_CK0: c_uint = 0x01C4;
pub const ixCLIENT2_CK1: c_uint = 0x01C8;
pub const ixCLIENT2_CK2: c_uint = 0x01CC;
pub const ixCLIENT2_CK3: c_uint = 0x01D0;
pub const ixCLIENT2_K0: c_uint = 0x01B4;
pub const ixCLIENT2_K1: c_uint = 0x01B8;
pub const ixCLIENT2_K2: c_uint = 0x01BC;
pub const ixCLIENT2_K3: c_uint = 0x01C0;
pub const ixCLIENT2_OFFSET: c_uint = 0x01E8;
pub const ixCLIENT2_OFFSET_HI: c_uint = 0x0298;
pub const ixCLIENT2_STATUS: c_uint = 0x01EC;
pub const ixCLIENT3_BM: c_uint = 0x02D4;
pub const ixCLIENT3_CD0: c_uint = 0x02C4;
pub const ixCLIENT3_CD1: c_uint = 0x02C8;
pub const ixCLIENT3_CD2: c_uint = 0x02CC;
pub const ixCLIENT3_CD3: c_uint = 0x02D0;
pub const ixCLIENT3_CK0: c_uint = 0x02B4;
pub const ixCLIENT3_CK1: c_uint = 0x02B8;
pub const ixCLIENT3_CK2: c_uint = 0x02BC;
pub const ixCLIENT3_CK3: c_uint = 0x02C0;
pub const ixCLIENT3_K0: c_uint = 0x02A4;
pub const ixCLIENT3_K1: c_uint = 0x02A8;
pub const ixCLIENT3_K2: c_uint = 0x02AC;
pub const ixCLIENT3_K3: c_uint = 0x02B0;
pub const ixCLIENT3_OFFSET: c_uint = 0x02D8;
pub const ixCLIENT3_OFFSET_HI: c_uint = 0x02A0;
pub const ixCLIENT3_STATUS: c_uint = 0x02DC;
pub const ixDH_TEST: c_uint = 0x0000;
pub const ixEXP0: c_uint = 0x0034;
pub const ixEXP1: c_uint = 0x0038;
pub const ixEXP2: c_uint = 0x003C;
pub const ixEXP3: c_uint = 0x0040;
pub const ixEXP4: c_uint = 0x0044;
pub const ixEXP5: c_uint = 0x0048;
pub const ixEXP6: c_uint = 0x004C;
pub const ixEXP7: c_uint = 0x0050;
pub const ixHFS_SEED0: c_uint = 0x0278;
pub const ixHFS_SEED1: c_uint = 0x027C;
pub const ixHFS_SEED2: c_uint = 0x0280;
pub const ixHFS_SEED3: c_uint = 0x0284;
pub const ixKEFUSE0: c_uint = 0x0268;
pub const ixKEFUSE1: c_uint = 0x026C;
pub const ixKEFUSE2: c_uint = 0x0270;
pub const ixKEFUSE3: c_uint = 0x0274;
pub const ixKHFS0: c_uint = 0x0004;
pub const ixKHFS1: c_uint = 0x0008;
pub const ixKHFS2: c_uint = 0x000C;
pub const ixKHFS3: c_uint = 0x0010;
pub const ixKSESSION0: c_uint = 0x0014;
pub const ixKSESSION1: c_uint = 0x0018;
pub const ixKSESSION2: c_uint = 0x001C;
pub const ixKSESSION3: c_uint = 0x0020;
pub const ixKSIG0: c_uint = 0x0024;
pub const ixKSIG1: c_uint = 0x0028;
pub const ixKSIG2: c_uint = 0x002C;
pub const ixKSIG3: c_uint = 0x0030;
pub const ixLX0: c_uint = 0x0054;
pub const ixLX1: c_uint = 0x0058;
pub const ixLX2: c_uint = 0x005C;
pub const ixLX3: c_uint = 0x0060;
pub const ixRINGOSC_MASK: c_uint = 0x0288;
pub const ixSPU_PORT_STATUS: c_uint = 0x029C;
pub const mmCC_DRM_ID_STRAPS: c_uint = 0x1559;
pub const mmCC_SYS_RB_BACKEND_DISABLE: c_uint = 0x03A0;
pub const mmCC_SYS_RB_REDUNDANCY: c_uint = 0x039F;
pub const mmCGTT_DRM_CLK_CTRL0: c_uint = 0x1579;
pub const mmCP_CONFIG: c_uint = 0x0F92;
pub const mmDC_TEST_DEBUG_DATA: c_uint = 0x157D;
pub const mmDC_TEST_DEBUG_INDEX: c_uint = 0x157C;
pub const mmGC_USER_SYS_RB_BACKEND_DISABLE: c_uint = 0x03A1;
pub const mmHDP_ADDR_CONFIG: c_uint = 0x0BD2;
pub const mmHDP_DEBUG0: c_uint = 0x0BCC;
pub const mmHDP_DEBUG1: c_uint = 0x0BCD;
pub const mmHDP_HOST_PATH_CNTL: c_uint = 0x0B00;
pub const mmHDP_LAST_SURFACE_HIT: c_uint = 0x0BCE;
pub const mmHDP_MEMIO_ADDR: c_uint = 0x0BF7;
pub const mmHDP_MEMIO_CNTL: c_uint = 0x0BF6;
pub const mmHDP_MEMIO_RD_DATA: c_uint = 0x0BFA;
pub const mmHDP_MEMIO_STATUS: c_uint = 0x0BF8;
pub const mmHDP_MEMIO_WR_DATA: c_uint = 0x0BF9;
pub const mmHDP_MEM_POWER_LS: c_uint = 0x0BD4;
pub const mmHDP_MISC_CNTL: c_uint = 0x0BD3;
pub const mmHDP_NONSURFACE_BASE: c_uint = 0x0B01;
pub const mmHDP_NONSURFACE_INFO: c_uint = 0x0B02;
pub const mmHDP_NONSURFACE_PREFETCH: c_uint = 0x0BD5;
pub const mmHDP_NONSURFACE_SIZE: c_uint = 0x0B03;
pub const mmHDP_NONSURF_FLAGS: c_uint = 0x0BC9;
pub const mmHDP_NONSURF_FLAGS_CLR: c_uint = 0x0BCA;
pub const mmHDP_OUTSTANDING_REQ: c_uint = 0x0BD1;
pub const mmHDP_SC_MULTI_CHIP_CNTL: c_uint = 0x0BD0;
pub const mmHDP_SW_SEMAPHORE: c_uint = 0x0BCB;
pub const mmHDP_TILING_CONFIG: c_uint = 0x0BCF;
pub const mmHDP_XDP_BARS_ADDR_39_36: c_uint = 0x0C44;
pub const mmHDP_XDP_BUSY_STS: c_uint = 0x0C3E;
pub const mmHDP_XDP_CGTT_BLK_CTRL: c_uint = 0x0C33;
pub const mmHDP_XDP_CHKN: c_uint = 0x0C40;
pub const mmHDP_XDP_D2H_BAR_UPDATE: c_uint = 0x0C02;
pub const mmHDP_XDP_D2H_FLUSH: c_uint = 0x0C01;
pub const mmHDP_XDP_D2H_RSVD_10: c_uint = 0x0C0A;
pub const mmHDP_XDP_D2H_RSVD_11: c_uint = 0x0C0B;
pub const mmHDP_XDP_D2H_RSVD_12: c_uint = 0x0C0C;
pub const mmHDP_XDP_D2H_RSVD_13: c_uint = 0x0C0D;
pub const mmHDP_XDP_D2H_RSVD_14: c_uint = 0x0C0E;
pub const mmHDP_XDP_D2H_RSVD_15: c_uint = 0x0C0F;
pub const mmHDP_XDP_D2H_RSVD_16: c_uint = 0x0C10;
pub const mmHDP_XDP_D2H_RSVD_17: c_uint = 0x0C11;
pub const mmHDP_XDP_D2H_RSVD_18: c_uint = 0x0C12;
pub const mmHDP_XDP_D2H_RSVD_19: c_uint = 0x0C13;
pub const mmHDP_XDP_D2H_RSVD_20: c_uint = 0x0C14;
pub const mmHDP_XDP_D2H_RSVD_21: c_uint = 0x0C15;
pub const mmHDP_XDP_D2H_RSVD_22: c_uint = 0x0C16;
pub const mmHDP_XDP_D2H_RSVD_23: c_uint = 0x0C17;
pub const mmHDP_XDP_D2H_RSVD_24: c_uint = 0x0C18;
pub const mmHDP_XDP_D2H_RSVD_25: c_uint = 0x0C19;
pub const mmHDP_XDP_D2H_RSVD_26: c_uint = 0x0C1A;
pub const mmHDP_XDP_D2H_RSVD_27: c_uint = 0x0C1B;
pub const mmHDP_XDP_D2H_RSVD_28: c_uint = 0x0C1C;
pub const mmHDP_XDP_D2H_RSVD_29: c_uint = 0x0C1D;
pub const mmHDP_XDP_D2H_RSVD_30: c_uint = 0x0C1E;
pub const mmHDP_XDP_D2H_RSVD_3: c_uint = 0x0C03;
pub const mmHDP_XDP_D2H_RSVD_31: c_uint = 0x0C1F;
pub const mmHDP_XDP_D2H_RSVD_32: c_uint = 0x0C20;
pub const mmHDP_XDP_D2H_RSVD_33: c_uint = 0x0C21;
pub const mmHDP_XDP_D2H_RSVD_34: c_uint = 0x0C22;
pub const mmHDP_XDP_D2H_RSVD_4: c_uint = 0x0C04;
pub const mmHDP_XDP_D2H_RSVD_5: c_uint = 0x0C05;
pub const mmHDP_XDP_D2H_RSVD_6: c_uint = 0x0C06;
pub const mmHDP_XDP_D2H_RSVD_7: c_uint = 0x0C07;
pub const mmHDP_XDP_D2H_RSVD_8: c_uint = 0x0C08;
pub const mmHDP_XDP_D2H_RSVD_9: c_uint = 0x0C09;
pub const mmHDP_XDP_DBG_ADDR: c_uint = 0x0C41;
pub const mmHDP_XDP_DBG_DATA: c_uint = 0x0C42;
pub const mmHDP_XDP_DBG_MASK: c_uint = 0x0C43;
pub const mmHDP_XDP_DIRECT2HDP_FIRST: c_uint = 0x0C00;
pub const mmHDP_XDP_DIRECT2HDP_LAST: c_uint = 0x0C23;
pub const mmHDP_XDP_FLUSH_ARMED_STS: c_uint = 0x0C3C;
pub const mmHDP_XDP_FLUSH_CNTR0_STS: c_uint = 0x0C3D;
pub const mmHDP_XDP_HDP_IPH_CFG: c_uint = 0x0C31;
pub const mmHDP_XDP_HDP_MBX_MC_CFG: c_uint = 0x0C2D;
pub const mmHDP_XDP_HDP_MC_CFG: c_uint = 0x0C2E;
pub const mmHDP_XDP_HST_CFG: c_uint = 0x0C2F;
pub const mmHDP_XDP_P2P_BAR0: c_uint = 0x0C34;
pub const mmHDP_XDP_P2P_BAR1: c_uint = 0x0C35;
pub const mmHDP_XDP_P2P_BAR2: c_uint = 0x0C36;
pub const mmHDP_XDP_P2P_BAR3: c_uint = 0x0C37;
pub const mmHDP_XDP_P2P_BAR4: c_uint = 0x0C38;
pub const mmHDP_XDP_P2P_BAR5: c_uint = 0x0C39;
pub const mmHDP_XDP_P2P_BAR6: c_uint = 0x0C3A;
pub const mmHDP_XDP_P2P_BAR7: c_uint = 0x0C3B;
pub const mmHDP_XDP_P2P_BAR_CFG: c_uint = 0x0C24;
pub const mmHDP_XDP_P2P_MBX_ADDR0: c_uint = 0x0C26;
pub const mmHDP_XDP_P2P_MBX_ADDR1: c_uint = 0x0C27;
pub const mmHDP_XDP_P2P_MBX_ADDR2: c_uint = 0x0C28;
pub const mmHDP_XDP_P2P_MBX_ADDR3: c_uint = 0x0C29;
pub const mmHDP_XDP_P2P_MBX_ADDR4: c_uint = 0x0C2A;
pub const mmHDP_XDP_P2P_MBX_ADDR5: c_uint = 0x0C2B;
pub const mmHDP_XDP_P2P_MBX_ADDR6: c_uint = 0x0C2C;
pub const mmHDP_XDP_P2P_MBX_OFFSET: c_uint = 0x0C25;
pub const mmHDP_XDP_SID_CFG: c_uint = 0x0C30;
pub const mmHDP_XDP_SRBM_CFG: c_uint = 0x0C32;
pub const mmHDP_XDP_STICKY: c_uint = 0x0C3F;
pub const mmIH_ADVFAULT_CNTL: c_uint = 0x0F8C;
pub const mmIH_CNTL: c_uint = 0x0F86;
pub const mmIH_LEVEL_STATUS: c_uint = 0x0F87;
pub const mmIH_PERFCOUNTER0_RESULT: c_uint = 0x0F8A;
pub const mmIH_PERFCOUNTER1_RESULT: c_uint = 0x0F8B;
pub const mmIH_PERFMON_CNTL: c_uint = 0x0F89;
pub const mmIH_RB_BASE: c_uint = 0x0F81;
pub const mmIH_RB_CNTL: c_uint = 0x0F80;
pub const mmIH_RB_RPTR: c_uint = 0x0F82;
pub const mmIH_RB_WPTR: c_uint = 0x0F83;
pub const mmIH_RB_WPTR_ADDR_HI: c_uint = 0x0F84;
pub const mmIH_RB_WPTR_ADDR_LO: c_uint = 0x0F85;
pub const mmIH_STATUS: c_uint = 0x0F88;
pub const mmDMA_GFX_RB_CNTL: c_uint = 0x3400;
pub const mmDMA_GFX_RB_BASE: c_uint = 0x3401;
pub const mmDMA_GFX_RB_RPTR: c_uint = 0x3402;
pub const mmDMA_GFX_RB_WPTR: c_uint = 0x3403;
pub const mmDMA_GFX_RB_RPTR_ADDR_HI: c_uint = 0x3407;
pub const mmDMA_GFX_RB_RPTR_ADDR_LO: c_uint = 0x3408;
pub const mmDMA_GFX_IB_CNTL: c_uint = 0x3409;
pub const mmDMA_GFX_IB_RPTR: c_uint = 0x340a;
pub const mmDMA_CNTL: c_uint = 0x340b;
pub const mmDMA_STATUS_REG: c_uint = 0x340D;
pub const mmDMA_TILING_CONFIG: c_uint = 0x342E;
pub const mmDMA_SEM_INCOMPLETE_TIMER_CNTL: c_uint = 0x3411;
pub const mmDMA_SEM_WAIT_FAIL_TIMER_CNTL: c_uint = 0x3412;
pub const mmDMA_POWER_CNTL: c_uint = 0x342F;
pub const mmDMA_CLK_CTRL: c_uint = 0x3430;
pub const mmDMA_PG: c_uint = 0x3435;
pub const mmDMA_PGFSM_CONFIG: c_uint = 0x3436;
pub const mmDMA_PGFSM_WRITE: c_uint = 0x3437;
pub const mmSEM_MAILBOX: c_uint = 0x0F9B;
pub const mmSEM_MAILBOX_CLIENTCONFIG: c_uint = 0x0F9A;
pub const mmSEM_MAILBOX_CONTROL: c_uint = 0x0F9C;
pub const mmSEM_MCIF_CONFIG: c_uint = 0x0F90;
pub const mmSRBM_CAM_DATA: c_uint = 0x0397;
pub const mmSRBM_CAM_INDEX: c_uint = 0x0396;
pub const mmSRBM_CHIP_REVISION: c_uint = 0x039B;
pub const mmSRBM_CNTL: c_uint = 0x0390;
pub const mmSRBM_DEBUG: c_uint = 0x03A4;
pub const mmSRBM_DEBUG_CNTL: c_uint = 0x0399;
pub const mmSRBM_DEBUG_DATA: c_uint = 0x039A;
pub const mmSRBM_DEBUG_SNAPSHOT: c_uint = 0x03A5;
pub const mmSRBM_GFX_CNTL: c_uint = 0x0391;
pub const mmSRBM_INT_ACK: c_uint = 0x03AA;
pub const mmSRBM_INT_CNTL: c_uint = 0x03A8;
pub const mmSRBM_INT_STATUS: c_uint = 0x03A9;
pub const mmSRBM_MC_CLKEN_CNTL: c_uint = 0x03B3;
pub const mmSRBM_PERFCOUNTER0_HI: c_uint = 0x0704;
pub const mmSRBM_PERFCOUNTER0_LO: c_uint = 0x0703;
pub const mmSRBM_PERFCOUNTER0_SELECT: c_uint = 0x0701;
pub const mmSRBM_PERFCOUNTER1_HI: c_uint = 0x0706;
pub const mmSRBM_PERFCOUNTER1_LO: c_uint = 0x0705;
pub const mmSRBM_PERFCOUNTER1_SELECT: c_uint = 0x0702;
pub const mmSRBM_PERFMON_CNTL: c_uint = 0x0700;
pub const mmSRBM_READ_ERROR: c_uint = 0x03A6;
pub const mmSRBM_SOFT_RESET: c_uint = 0x0398;
pub const mmSRBM_STATUS: c_uint = 0x0394;
pub const mmSRBM_STATUS2: c_uint = 0x0393;
pub const mmSRBM_SYS_CLKEN_CNTL: c_uint = 0x03B4;
pub const mmSRBM_UVD_CLKEN_CNTL: c_uint = 0x03B6;
pub const mmSRBM_VCE_CLKEN_CNTL: c_uint = 0x03B5;
pub const mmUVD_CONFIG: c_uint = 0x0F98;
pub const mmVCE_CONFIG: c_uint = 0x0F94;
pub const mmXDMA_MSTR_MEM_OVERFLOW_CNTL: c_uint = 0x03F8;
