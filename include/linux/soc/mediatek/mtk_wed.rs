//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/mediatek/mtk_wed.h
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


pub const MTK_WED_TX_QUEUES: c_int = 2;
pub const MTK_WED_RX_QUEUES: c_int = 2;
pub const MTK_WED_RX_PAGE_QUEUES: c_int = 3;
pub const WED_WO_STA_REC: c_uint = 0x6;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_wed_wo_cmd {
    MTK_WED_WO_CMD_WED_CFG,
    MTK_WED_WO_CMD_WED_RX_STAT,
    MTK_WED_WO_CMD_RRO_SER,
    MTK_WED_WO_CMD_DBG_INFO,
    MTK_WED_WO_CMD_DEV_INFO,
    MTK_WED_WO_CMD_BSS_INFO,
    MTK_WED_WO_CMD_STA_REC,
    MTK_WED_WO_CMD_DEV_INFO_DUMP,
    MTK_WED_WO_CMD_BSS_INFO_DUMP,
    MTK_WED_WO_CMD_STA_REC_DUMP,
    MTK_WED_WO_CMD_BA_INFO_DUMP,
    MTK_WED_WO_CMD_FBCMD_Q_DUMP,
    MTK_WED_WO_CMD_FW_LOG_CTRL,
    MTK_WED_WO_CMD_LOG_FLUSH,
    MTK_WED_WO_CMD_CHANGE_STATE,
    MTK_WED_WO_CMD_CPU_STATS_ENABLE,
    MTK_WED_WO_CMD_CPU_STATS_DUMP,
    MTK_WED_WO_CMD_EXCEPTION_INIT,
    MTK_WED_WO_CMD_PROF_CTRL,
    MTK_WED_WO_CMD_STA_BA_DUMP,
    MTK_WED_WO_CMD_BA_CTRL_DUMP,
    MTK_WED_WO_CMD_RXCNT_CTRL,
    MTK_WED_WO_CMD_RXCNT_INFO,
    MTK_WED_WO_CMD_SET_CAP,
    MTK_WED_WO_CMD_CCIF_RING_DUMP,
    MTK_WED_WO_CMD_WED_END
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_wed_bm_desc {
    pub buf0: __le32,
    pub token: __le32,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_wed_bus_tye {
    MTK_WED_BUS_PCIE,
    MTK_WED_BUS_AXI,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_wed_ring {
    pub desc: *mut mtk_wdma_desc,
    pub desc_phys: dma_addr_t,
    pub desc_size: u32,
    pub size: c_int,
    pub flags: u32,
    pub reg_base: u32,
    pub wpdma: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_wed_wo_rx_stats {
    pub wlan_idx: __le16,
    pub tid: __le16,
    pub rx_pkt_cnt: __le32,
    pub rx_byte_cnt: __le32,
    pub rx_err_cnt: __le32,
    pub rx_drop_cnt: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_wed_buf {
    pub p: *mut c_void,
    pub phy_addr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_wed_device {

    pub ops: *const mtk_wed_ops,
    pub dev: *mut device,
    pub hw: *mut mtk_wed_hw,
    pub running: bool init_done,,
    pub wdma_idx: c_int,
    pub irq: c_int,
    pub version: u8,
// used by wlan driver
    pub rev_id: u32,
    pub tx_ring: [mtk_wed_ring; MTK_WED_TX_QUEUES],
    pub rx_ring: [mtk_wed_ring; MTK_WED_RX_QUEUES],
    pub txfree_ring: mtk_wed_ring,
    pub tx_wdma: [mtk_wed_ring; MTK_WED_TX_QUEUES],
    pub rx_wdma: [mtk_wed_ring; MTK_WED_RX_QUEUES],
    pub rx_rro_ring: [mtk_wed_ring; MTK_WED_RX_QUEUES],
    pub rx_page_ring: [mtk_wed_ring; MTK_WED_RX_PAGE_QUEUES],
    pub ind_cmd_ring: mtk_wed_ring,
    pub size: c_int,
    pub pages: *mut mtk_wed_buf,
    pub desc: *mut mtk_wdma_desc,
    pub desc_phys: dma_addr_t,
    pub tx_buf_ring: },
    pub size: c_int,
    pub desc: *mut mtk_wed_bm_desc,
    pub desc_phys: dma_addr_t,
    pub rx_buf_ring: },
    pub ring: mtk_wed_ring,
    pub miod_phys: dma_addr_t,
    pub fdbk_phys: dma_addr_t,
    pub rro: },
    pub size: c_int,
    pub pages: *mut mtk_wed_buf,
    pub desc: *mut mtk_wed_bm_desc,
    pub desc_phys: dma_addr_t,
    pub hw_rro: },
// filled by driver:
    pub platform_dev: *mut platform_device,
    pub pci_dev: *mut pci_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_wed_ops {
    pub __releases(RCU): *mut *mut *mut int (attach)(struct mtk_wed_device dev),
    pub reset): *mut *mut void __iomem regs, bool,
    pub reset): *mut *mut void __iomem regs, bool,
    pub regs): *mut void __iomem,
    pub len): *mut *mut void data, int,
    pub dev): *mut *mut void (detach)(struct mtk_wed_device,
    pub hash): u32 reason, u32,
    pub dev): *mut *mut void (stop)(struct mtk_wed_device,
    pub irq_mask): *mut *mut *mut void (start)(struct mtk_wed_device dev, u32,
    pub dev): *mut *mut void (reset_dma)(struct mtk_wed_device,
    pub reg): *mut *mut *mut u32 (reg_read)(struct mtk_wed_device dev, u32,
    pub val): *mut *mut *mut void (reg_write)(struct mtk_wed_device dev, u32 reg, u32,
    pub mask): *mut *mut *mut u32 (irq_get)(struct mtk_wed_device dev, u32,
    pub mask): *mut *mut *mut void (irq_set_mask)(struct mtk_wed_device dev, u32,
    pub type_data): *mut tc_setup_type type, void,
    pub reset): bool,
    pub regs): *mut void __iomem,
    pub regs): *mut void __iomem,
    pub regs): *mut void __iomem,
}

