//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/freescale/fec_mpc52xx.h
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
// drivers/net/ethernet/freescale/fec_mpc52xx.h
//
// Driver for the MPC5200 Fast Ethernet Controller
//
// Author: Dale Farnsworth <dfarnsworth@mvista.com>
//
// 2003-2004 (c) MontaVista, Software, Inc.  This file is licensed under
// the terms of the GNU General Public License version 2.  This program
// is licensed "as is" without any warranty of any kind, whether express
// or implied.
//

// Tunable constant
// FEC_RX_BUFFER_SIZE includes 4 bytes for CRC32

pub const FEC_RX_NUM_BD: c_int = 256;
pub const FEC_TX_NUM_BD: c_int = 64;

// ========================================================================
// Hardware register sets & bits
// ========================================================================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc52xx_fec {
    pub /: *mut *mut u32 fec_id; / FEC + 0x000,
    pub /: *mut *mut u32 ievent; / FEC + 0x004,
    pub /: *mut *mut u32 imask; / FEC + 0x008,
    pub /: *mut *mut u32 reserved0[1]; / FEC + 0x00C,
    pub /: *mut *mut u32 r_des_active; / FEC + 0x010,
    pub /: *mut *mut u32 x_des_active; / FEC + 0x014,
    pub /: *mut *mut u32 r_des_active_cl; / FEC + 0x018,
    pub /: *mut *mut u32 x_des_active_cl; / FEC + 0x01C,
    pub /: *mut *mut u32 ivent_set; / FEC + 0x020,
    pub /: *mut *mut u32 ecntrl; / FEC + 0x024,
    pub /: *mut *mut u32 reserved1[6]; / FEC + 0x028-03C,
    pub /: *mut *mut u32 mii_data; / FEC + 0x040,
    pub /: *mut *mut u32 mii_speed; / FEC + 0x044,
    pub /: *mut *mut u32 mii_status; / FEC + 0x048,
    pub /: *mut *mut u32 reserved2[5]; / FEC + 0x04C-05C,
    pub /: *mut *mut u32 mib_data; / FEC + 0x060,
    pub /: *mut *mut u32 mib_control; / FEC + 0x064,
    pub /: *mut *mut u32 reserved3[6]; / FEC + 0x068-7C,
    pub /: *mut *mut u32 r_activate; / FEC + 0x080,
    pub /: *mut *mut u32 r_cntrl; / FEC + 0x084,
    pub /: *mut *mut u32 r_hash; / FEC + 0x088,
    pub /: *mut *mut u32 r_data; / FEC + 0x08C,
    pub /: *mut *mut u32 ar_done; / FEC + 0x090,
    pub /: *mut *mut u32 r_test; / FEC + 0x094,
    pub /: *mut *mut u32 r_mib; / FEC + 0x098,
    pub /: *mut *mut u32 r_da_low; / FEC + 0x09C,
    pub /: *mut *mut u32 r_da_high; / FEC + 0x0A0,
    pub /: *mut *mut u32 reserved4[7]; / FEC + 0x0A4-0BC,
    pub /: *mut *mut u32 x_activate; / FEC + 0x0C0,
    pub /: *mut *mut u32 x_cntrl; / FEC + 0x0C4,
    pub /: *mut *mut u32 backoff; / FEC + 0x0C8,
    pub /: *mut *mut u32 x_data; / FEC + 0x0CC,
    pub /: *mut *mut u32 x_status; / FEC + 0x0D0,
    pub /: *mut *mut u32 x_mib; / FEC + 0x0D4,
    pub /: *mut *mut u32 x_test; / FEC + 0x0D8,
    pub /: *mut *mut u32 fdxfc_da1; / FEC + 0x0DC,
    pub /: *mut *mut u32 fdxfc_da2; / FEC + 0x0E0,
    pub /: *mut *mut u32 paddr1; / FEC + 0x0E4,
    pub /: *mut *mut u32 paddr2; / FEC + 0x0E8,
    pub /: *mut *mut u32 op_pause; / FEC + 0x0EC,
    pub /: *mut *mut u32 reserved5[4]; / FEC + 0x0F0-0FC,
    pub /: *mut *mut u32 instr_reg; / FEC + 0x100,
    pub /: *mut *mut u32 context_reg; / FEC + 0x104,
    pub /: *mut *mut u32 test_cntrl; / FEC + 0x108,
    pub /: *mut *mut u32 acc_reg; / FEC + 0x10C,
    pub /: *mut *mut u32 ones; / FEC + 0x110,
    pub /: *mut *mut u32 zeros; / FEC + 0x114,
    pub /: *mut *mut u32 iaddr1; / FEC + 0x118,
    pub /: *mut *mut u32 iaddr2; / FEC + 0x11C,
    pub /: *mut *mut u32 gaddr1; / FEC + 0x120,
    pub /: *mut *mut u32 gaddr2; / FEC + 0x124,
    pub /: *mut *mut u32 random; / FEC + 0x128,
    pub /: *mut *mut u32 rand1; / FEC + 0x12C,
    pub /: *mut *mut u32 tmp; / FEC + 0x130,
    pub /: *mut *mut u32 reserved6[3]; / FEC + 0x134-13C,
    pub /: *mut *mut u32 fifo_id; / FEC + 0x140,
    pub /: *mut *mut u32 x_wmrk; / FEC + 0x144,
    pub /: *mut *mut u32 fcntrl; / FEC + 0x148,
    pub /: *mut *mut u32 r_bound; / FEC + 0x14C,
    pub /: *mut *mut u32 r_fstart; / FEC + 0x150,
    pub /: *mut *mut u32 r_count; / FEC + 0x154,
    pub /: *mut *mut u32 r_lag; / FEC + 0x158,
    pub /: *mut *mut u32 r_read; / FEC + 0x15C,
    pub /: *mut *mut u32 r_write; / FEC + 0x160,
    pub /: *mut *mut u32 x_count; / FEC + 0x164,
    pub /: *mut *mut u32 x_lag; / FEC + 0x168,
    pub /: *mut *mut u32 x_retry; / FEC + 0x16C,
    pub /: *mut *mut u32 x_write; / FEC + 0x170,
    pub /: *mut *mut u32 x_read; / FEC + 0x174,
    pub /: *mut *mut u32 reserved7[2]; / FEC + 0x178-17C,
    pub /: *mut *mut u32 fm_cntrl; / FEC + 0x180,
    pub /: *mut *mut u32 rfifo_data; / FEC + 0x184,
    pub /: *mut *mut u32 rfifo_status; / FEC + 0x188,
    pub /: *mut *mut u32 rfifo_cntrl; / FEC + 0x18C,
    pub /: *mut *mut u32 rfifo_lrf_ptr; / FEC + 0x190,
    pub /: *mut *mut u32 rfifo_lwf_ptr; / FEC + 0x194,
    pub /: *mut *mut u32 rfifo_alarm; / FEC + 0x198,
    pub /: *mut *mut u32 rfifo_rdptr; / FEC + 0x19C,
    pub /: *mut *mut u32 rfifo_wrptr; / FEC + 0x1A0,
    pub /: *mut *mut u32 tfifo_data; / FEC + 0x1A4,
    pub /: *mut *mut u32 tfifo_status; / FEC + 0x1A8,
    pub /: *mut *mut u32 tfifo_cntrl; / FEC + 0x1AC,
    pub /: *mut *mut u32 tfifo_lrf_ptr; / FEC + 0x1B0,
    pub /: *mut *mut u32 tfifo_lwf_ptr; / FEC + 0x1B4,
    pub /: *mut *mut u32 tfifo_alarm; / FEC + 0x1B8,
    pub /: *mut *mut u32 tfifo_rdptr; / FEC + 0x1BC,
    pub /: *mut *mut u32 tfifo_wrptr; / FEC + 0x1C0,
    pub /: *mut *mut u32 reset_cntrl; / FEC + 0x1C4,
    pub /: *mut *mut u32 xmit_fsm; / FEC + 0x1C8,
    pub /: *mut *mut u32 reserved8[3]; / FEC + 0x1CC-1D4,
    pub /: *mut *mut u32 rdes_data0; / FEC + 0x1D8,
    pub /: *mut *mut u32 rdes_data1; / FEC + 0x1DC,
    pub /: *mut *mut u32 r_length; / FEC + 0x1E0,
    pub /: *mut *mut u32 x_length; / FEC + 0x1E4,
    pub /: *mut *mut u32 x_addr; / FEC + 0x1E8,
    pub /: *mut *mut u32 cdes_data; / FEC + 0x1EC,
    pub /: *mut *mut u32 status; / FEC + 0x1F0,
    pub /: *mut *mut u32 dma_control; / FEC + 0x1F4,
    pub /: *mut *mut u32 des_cmnd; / FEC + 0x1F8,
    pub /: *mut *mut u32 data; / FEC + 0x1FC,
    pub /: *mut *mut u32 rmon_t_drop; / FEC + 0x200,
    pub /: *mut *mut u32 rmon_t_packets; / FEC + 0x204,
    pub /: *mut *mut u32 rmon_t_bc_pkt; / FEC + 0x208,
    pub /: *mut *mut u32 rmon_t_mc_pkt; / FEC + 0x20C,
    pub /: *mut *mut u32 rmon_t_crc_align; / FEC + 0x210,
    pub /: *mut *mut u32 rmon_t_undersize; / FEC + 0x214,
    pub /: *mut *mut u32 rmon_t_oversize; / FEC + 0x218,
    pub /: *mut *mut u32 rmon_t_frag; / FEC + 0x21C,
    pub /: *mut *mut u32 rmon_t_jab; / FEC + 0x220,
    pub /: *mut *mut u32 rmon_t_col; / FEC + 0x224,
    pub /: *mut *mut u32 rmon_t_p64; / FEC + 0x228,
    pub /: *mut *mut u32 rmon_t_p65to127; / FEC + 0x22C,
    pub /: *mut *mut u32 rmon_t_p128to255; / FEC + 0x230,
    pub /: *mut *mut u32 rmon_t_p256to511; / FEC + 0x234,
    pub /: *mut *mut u32 rmon_t_p512to1023; / FEC + 0x238,
    pub /: *mut *mut u32 rmon_t_p1024to2047; / FEC + 0x23C,
    pub /: *mut *mut u32 rmon_t_p_gte2048; / FEC + 0x240,
    pub /: *mut *mut u32 rmon_t_octets; / FEC + 0x244,
    pub /: *mut *mut u32 ieee_t_drop; / FEC + 0x248,
    pub /: *mut *mut u32 ieee_t_frame_ok; / FEC + 0x24C,
    pub /: *mut *mut u32 ieee_t_1col; / FEC + 0x250,
    pub /: *mut *mut u32 ieee_t_mcol; / FEC + 0x254,
    pub /: *mut *mut u32 ieee_t_def; / FEC + 0x258,
    pub /: *mut *mut u32 ieee_t_lcol; / FEC + 0x25C,
    pub /: *mut *mut u32 ieee_t_excol; / FEC + 0x260,
    pub /: *mut *mut u32 ieee_t_macerr; / FEC + 0x264,
    pub /: *mut *mut u32 ieee_t_cserr; / FEC + 0x268,
    pub /: *mut *mut u32 ieee_t_sqe; / FEC + 0x26C,
    pub /: *mut *mut u32 t_fdxfc; / FEC + 0x270,
    pub /: *mut *mut u32 ieee_t_octets_ok; / FEC + 0x274,
    pub /: *mut *mut u32 reserved9[2]; / FEC + 0x278-27C,
    pub /: *mut *mut u32 rmon_r_drop; / FEC + 0x280,
    pub /: *mut *mut u32 rmon_r_packets; / FEC + 0x284,
    pub /: *mut *mut u32 rmon_r_bc_pkt; / FEC + 0x288,
    pub /: *mut *mut u32 rmon_r_mc_pkt; / FEC + 0x28C,
    pub /: *mut *mut u32 rmon_r_crc_align; / FEC + 0x290,
    pub /: *mut *mut u32 rmon_r_undersize; / FEC + 0x294,
    pub /: *mut *mut u32 rmon_r_oversize; / FEC + 0x298,
    pub /: *mut *mut u32 rmon_r_frag; / FEC + 0x29C,
    pub /: *mut *mut u32 rmon_r_jab; / FEC + 0x2A0,
    pub /: *mut *mut u32 rmon_r_resvd_0; / FEC + 0x2A4,
    pub /: *mut *mut u32 rmon_r_p64; / FEC + 0x2A8,
    pub /: *mut *mut u32 rmon_r_p65to127; / FEC + 0x2AC,
    pub /: *mut *mut u32 rmon_r_p128to255; / FEC + 0x2B0,
    pub /: *mut *mut u32 rmon_r_p256to511; / FEC + 0x2B4,
    pub /: *mut *mut u32 rmon_r_p512to1023; / FEC + 0x2B8,
    pub /: *mut *mut u32 rmon_r_p1024to2047; / FEC + 0x2BC,
    pub /: *mut *mut u32 rmon_r_p_gte2048; / FEC + 0x2C0,
    pub /: *mut *mut u32 rmon_r_octets; / FEC + 0x2C4,
    pub /: *mut *mut u32 ieee_r_drop; / FEC + 0x2C8,
    pub /: *mut *mut u32 ieee_r_frame_ok; / FEC + 0x2CC,
    pub /: *mut *mut u32 ieee_r_crc; / FEC + 0x2D0,
    pub /: *mut *mut u32 ieee_r_align; / FEC + 0x2D4,
    pub /: *mut *mut u32 r_macerr; / FEC + 0x2D8,
    pub /: *mut *mut u32 r_fdxfc; / FEC + 0x2DC,
    pub /: *mut *mut u32 ieee_r_octets_ok; / FEC + 0x2E0,
    pub /: *mut *mut u32 reserved10[7]; / FEC + 0x2E4-2FC,
    pub /: *mut *mut u32 reserved11[64]; / FEC + 0x300-3FF,
}

pub const FEC_MIB_DISABLE: c_uint = 0x80000000;
pub const FEC_IEVENT_HBERR: c_uint = 0x80000000;
pub const FEC_IEVENT_BABR: c_uint = 0x40000000;
pub const FEC_IEVENT_BABT: c_uint = 0x20000000;
pub const FEC_IEVENT_GRA: c_uint = 0x10000000;
pub const FEC_IEVENT_TFINT: c_uint = 0x08000000;
pub const FEC_IEVENT_MII: c_uint = 0x00800000;
pub const FEC_IEVENT_LATE_COL: c_uint = 0x00200000;
pub const FEC_IEVENT_COL_RETRY_LIM: c_uint = 0x00100000;
pub const FEC_IEVENT_XFIFO_UN: c_uint = 0x00080000;
pub const FEC_IEVENT_XFIFO_ERROR: c_uint = 0x00040000;
pub const FEC_IEVENT_RFIFO_ERROR: c_uint = 0x00020000;
pub const FEC_IMASK_HBERR: c_uint = 0x80000000;
pub const FEC_IMASK_BABR: c_uint = 0x40000000;
pub const FEC_IMASK_BABT: c_uint = 0x20000000;
pub const FEC_IMASK_GRA: c_uint = 0x10000000;
pub const FEC_IMASK_MII: c_uint = 0x00800000;
pub const FEC_IMASK_LATE_COL: c_uint = 0x00200000;
pub const FEC_IMASK_COL_RETRY_LIM: c_uint = 0x00100000;
pub const FEC_IMASK_XFIFO_UN: c_uint = 0x00080000;
pub const FEC_IMASK_XFIFO_ERROR: c_uint = 0x00040000;
pub const FEC_IMASK_RFIFO_ERROR: c_uint = 0x00020000;
// all but MII, which is enabled separately

pub const FEC_RCNTRL_MAX_FL_SHIFT: c_int = 16;
pub const FEC_RCNTRL_LOOP: c_uint = 0x01;
pub const FEC_RCNTRL_DRT: c_uint = 0x02;
pub const FEC_RCNTRL_MII_MODE: c_uint = 0x04;
pub const FEC_RCNTRL_PROM: c_uint = 0x08;
pub const FEC_RCNTRL_BC_REJ: c_uint = 0x10;
pub const FEC_RCNTRL_FCE: c_uint = 0x20;
pub const FEC_TCNTRL_GTS: c_uint = 0x00000001;
pub const FEC_TCNTRL_HBC: c_uint = 0x00000002;
pub const FEC_TCNTRL_FDEN: c_uint = 0x00000004;
pub const FEC_TCNTRL_TFC_PAUSE: c_uint = 0x00000008;
pub const FEC_TCNTRL_RFC_PAUSE: c_uint = 0x00000010;
pub const FEC_ECNTRL_RESET: c_uint = 0x00000001;
pub const FEC_ECNTRL_ETHER_EN: c_uint = 0x00000002;
pub const FEC_MII_DATA_ST: c_uint = 0x40000000	/* Start frame */;
pub const FEC_MII_DATA_OP_RD: c_uint = 0x20000000	/* Perform read */;
pub const FEC_MII_DATA_OP_WR: c_uint = 0x10000000	/* Perform write */;
pub const FEC_MII_DATA_PA_MSK: c_uint = 0x0f800000	/* PHY Address mask */;
pub const FEC_MII_DATA_RA_MSK: c_uint = 0x007c0000	/* PHY Register mask */;
pub const FEC_MII_DATA_TA: c_uint = 0x00020000	/* Turnaround */;
pub const FEC_MII_DATA_DATAMSK: c_uint = 0x0000ffff	/* PHY data mask */;

pub const FEC_MII_DATA_RA_SHIFT: c_uint = 0x12		/* MII reg addr bits */;
pub const FEC_MII_DATA_PA_SHIFT: c_uint = 0x17		/* MII PHY addr bits */;
pub const FEC_PADDR2_TYPE: c_uint = 0x8808;
pub const FEC_OP_PAUSE_OPCODE: c_uint = 0x00010000;
pub const FEC_FIFO_WMRK_256B: c_uint = 0x3;
pub const FEC_FIFO_STATUS_ERR: c_uint = 0x00400000;
pub const FEC_FIFO_STATUS_UF: c_uint = 0x00200000;
pub const FEC_FIFO_STATUS_OF: c_uint = 0x00100000;
pub const FEC_FIFO_CNTRL_FRAME: c_uint = 0x08000000;
pub const FEC_FIFO_CNTRL_LTG_7: c_uint = 0x07000000;
pub const FEC_RESET_CNTRL_RESET_FIFO: c_uint = 0x02000000;
pub const FEC_RESET_CNTRL_ENABLE_IS_RESET: c_uint = 0x01000000;
pub const FEC_XMIT_FSM_APPEND_CRC: c_uint = 0x02000000;
pub const FEC_XMIT_FSM_ENABLE_CRC: c_uint = 0x01000000;
