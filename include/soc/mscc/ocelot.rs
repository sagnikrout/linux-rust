//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/mscc/ocelot.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
// Copyright (c) 2017 Microsemi Corporation
//

// Port Group IDs (PGID) are masks of destination ports.
//
// For L2 forwarding, the switch performs 3 lookups in the PGID table for each
// frame, and forwards the frame to the ports that are present in the logical
// AND of all 3 PGIDs.
//
// These PGID lookups are:
// - In one of PGID[0-63]: for the destination masks. There are 2 paths by
// which the switch selects a destination PGID:
// - The {DMAC, VID} is present in the MAC table. In that case, the
// destination PGID is given by the DEST_IDX field of the MAC table entry
// that matched.
// - The {DMAC, VID} is not present in the MAC table (it is unknown). The
// frame is disseminated as being either unicast, multicast or broadcast,
// and according to that, the destination PGID is chosen as being the
// value contained by ANA_FLOODING_FLD_UNICAST,
// ANA_FLOODING_FLD_MULTICAST or ANA_FLOODING_FLD_BROADCAST.
// The destination PGID can be an unicast set: the first PGIDs, 0 to
// ocelot->num_phys_ports - 1, or a multicast set: the PGIDs from
// ocelot->num_phys_ports to 63. By convention, a unicast PGID corresponds to
// a physical port and has a single bit set in the destination ports mask:
// that corresponding to the port number itself. In contrast, a multicast
// PGID will have potentially more than one single bit set in the destination
// ports mask.
// - In one of PGID[64-79]: for the aggregation mask. The switch classifier
// dissects each frame and generates a 4-bit Link Aggregation Code which is
// used for this second PGID table lookup. The goal of link aggregation is to
// hash multiple flows within the same LAG on to different destination ports.
// The first lookup will result in a PGID with all the LAG members present in
// the destination ports mask, and the second lookup, by Link Aggregation
// Code, will ensure that each flow gets forwarded only to a single port out
// of that mask (there are no duplicates).
// - In one of PGID[80-90]: for the source mask. The third time, the PGID table
// is indexed with the ingress port (plus 80). These PGIDs answer the
// question "is port i allowed to forward traffic to port j?" If yes, then
// BIT(j) of PGID 80+i will be found set. The third PGID lookup can be used
// to enforce the L2 forwarding matrix imposed by e.g. a Linux bridge.
//
// Reserve some destination PGIDs at the end of the range:
// PGID_BLACKHOLE: used for not forwarding the frames
// PGID_CPU: used for whitelisting certain MAC addresses, such as the addresses
// of the switch port net devices, towards the CPU port module.
// PGID_UC: the flooding destinations for unknown unicast traffic.
// PGID_MC: the flooding destinations for non-IP multicast traffic.
// PGID_MCIPV4: the flooding destinations for IPv4 multicast traffic.
// PGID_MCIPV6: the flooding destinations for IPv6 multicast traffic.
// PGID_BC: the flooding destinations for broadcast traffic.
//
pub const PGID_BLACKHOLE: c_int = 57;
pub const PGID_CPU: c_int = 58;
pub const PGID_UC: c_int = 59;
pub const PGID_MC: c_int = 60;
pub const PGID_MCIPV4: c_int = 61;
pub const PGID_MCIPV6: c_int = 62;
pub const PGID_BC: c_int = 63;

// Aggregation PGIDs, one per Link Aggregation Code
pub const PGID_AGGR: c_int = 64;
// Source PGIDs, one per physical port
pub const PGID_SRC: c_int = 80;
pub const OCELOT_NUM_TC: c_int = 8;
pub const OCELOT_SPEED_2500: c_int = 0;
pub const OCELOT_SPEED_1000: c_int = 1;
pub const OCELOT_SPEED_100: c_int = 2;
pub const OCELOT_SPEED_10: c_int = 3;
pub const OCELOT_PTP_PINS_NUM: c_int = 4;
pub const TARGET_OFFSET: c_int = 24;

pub const REG_RESERVED_ADDR: c_uint = 0xffffffff;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocelot_target {
    ANA = 1,
    QS,
    QSYS,
    REW,
    SYS,
    S0,
    S1,
    S2,
    HSIO,
    PTP,
    FDMA,
    GCB,
    DEV_GMII,
    TARGET_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocelot_reg {
    ANA_ADVLEARN = ANA << TARGET_OFFSET,
    ANA_VLANMASK,
    ANA_PORT_B_DOMAIN,
    ANA_ANAGEFIL,
    ANA_ANEVENTS,
    ANA_STORMLIMIT_BURST,
    ANA_STORMLIMIT_CFG,
    ANA_ISOLATED_PORTS,
    ANA_COMMUNITY_PORTS,
    ANA_AUTOAGE,
    ANA_MACTOPTIONS,
    ANA_LEARNDISC,
    ANA_AGENCTRL,
    ANA_MIRRORPORTS,
    ANA_EMIRRORPORTS,
    ANA_FLOODING,
    ANA_FLOODING_IPMC,
    ANA_SFLOW_CFG,
    ANA_PORT_MODE,
    ANA_CUT_THRU_CFG,
    ANA_PGID_PGID,
    ANA_TABLES_ANMOVED,
    ANA_TABLES_MACHDATA,
    ANA_TABLES_MACLDATA,
    ANA_TABLES_STREAMDATA,
    ANA_TABLES_MACACCESS,
    ANA_TABLES_MACTINDX,
    ANA_TABLES_VLANACCESS,
    ANA_TABLES_VLANTIDX,
    ANA_TABLES_ISDXACCESS,
    ANA_TABLES_ISDXTIDX,
    ANA_TABLES_ENTRYLIM,
    ANA_TABLES_PTP_ID_HIGH,
    ANA_TABLES_PTP_ID_LOW,
    ANA_TABLES_STREAMACCESS,
    ANA_TABLES_STREAMTIDX,
    ANA_TABLES_SEQ_HISTORY,
    ANA_TABLES_SEQ_MASK,
    ANA_TABLES_SFID_MASK,
    ANA_TABLES_SFIDACCESS,
    ANA_TABLES_SFIDTIDX,
    ANA_MSTI_STATE,
    ANA_OAM_UPM_LM_CNT,
    ANA_SG_ACCESS_CTRL,
    ANA_SG_CONFIG_REG_1,
    ANA_SG_CONFIG_REG_2,
    ANA_SG_CONFIG_REG_3,
    ANA_SG_CONFIG_REG_4,
    ANA_SG_CONFIG_REG_5,
    ANA_SG_GCL_GS_CONFIG,
    ANA_SG_GCL_TI_CONFIG,
    ANA_SG_STATUS_REG_1,
    ANA_SG_STATUS_REG_2,
    ANA_SG_STATUS_REG_3,
    ANA_PORT_VLAN_CFG,
    ANA_PORT_DROP_CFG,
    ANA_PORT_QOS_CFG,
    ANA_PORT_VCAP_CFG,
    ANA_PORT_VCAP_S1_KEY_CFG,
    ANA_PORT_VCAP_S2_CFG,
    ANA_PORT_PCP_DEI_MAP,
    ANA_PORT_CPU_FWD_CFG,
    ANA_PORT_CPU_FWD_BPDU_CFG,
    ANA_PORT_CPU_FWD_GARP_CFG,
    ANA_PORT_CPU_FWD_CCM_CFG,
    ANA_PORT_PORT_CFG,
    ANA_PORT_POL_CFG,
    ANA_PORT_PTP_CFG,
    ANA_PORT_PTP_DLY1_CFG,
    ANA_PORT_PTP_DLY2_CFG,
    ANA_PORT_SFID_CFG,
    ANA_PFC_PFC_CFG,
    ANA_PFC_PFC_TIMER,
    ANA_IPT_OAM_MEP_CFG,
    ANA_IPT_IPT,
    ANA_PPT_PPT,
    ANA_FID_MAP_FID_MAP,
    ANA_AGGR_CFG,
    ANA_CPUQ_CFG,
    ANA_CPUQ_CFG2,
    ANA_CPUQ_8021_CFG,
    ANA_DSCP_CFG,
    ANA_DSCP_REWR_CFG,
    ANA_VCAP_RNG_TYPE_CFG,
    ANA_VCAP_RNG_VAL_CFG,
    ANA_VRAP_CFG,
    ANA_VRAP_HDR_DATA,
    ANA_VRAP_HDR_MASK,
    ANA_DISCARD_CFG,
    ANA_FID_CFG,
    ANA_POL_PIR_CFG,
    ANA_POL_CIR_CFG,
    ANA_POL_MODE_CFG,
    ANA_POL_PIR_STATE,
    ANA_POL_CIR_STATE,
    ANA_POL_STATE,
    ANA_POL_FLOWC,
    ANA_POL_HYST,
    ANA_POL_MISC_CFG,
    QS_XTR_GRP_CFG = QS << TARGET_OFFSET,
    QS_XTR_RD,
    QS_XTR_FRM_PRUNING,
    QS_XTR_FLUSH,
    QS_XTR_DATA_PRESENT,
    QS_XTR_CFG,
    QS_INJ_GRP_CFG,
    QS_INJ_WR,
    QS_INJ_CTRL,
    QS_INJ_STATUS,
    QS_INJ_ERR,
    QS_INH_DBG,
    QSYS_PORT_MODE = QSYS << TARGET_OFFSET,
    QSYS_SWITCH_PORT_MODE,
    QSYS_STAT_CNT_CFG,
    QSYS_EEE_CFG,
    QSYS_EEE_THRES,
    QSYS_IGR_NO_SHARING,
    QSYS_EGR_NO_SHARING,
    QSYS_SW_STATUS,
    QSYS_EXT_CPU_CFG,
    QSYS_PAD_CFG,
    QSYS_CPU_GROUP_MAP,
    QSYS_QMAP,
    QSYS_ISDX_SGRP,
    QSYS_TIMED_FRAME_ENTRY,
    QSYS_TFRM_MISC,
    QSYS_TFRM_PORT_DLY,
    QSYS_TFRM_TIMER_CFG_1,
    QSYS_TFRM_TIMER_CFG_2,
    QSYS_TFRM_TIMER_CFG_3,
    QSYS_TFRM_TIMER_CFG_4,
    QSYS_TFRM_TIMER_CFG_5,
    QSYS_TFRM_TIMER_CFG_6,
    QSYS_TFRM_TIMER_CFG_7,
    QSYS_TFRM_TIMER_CFG_8,
    QSYS_RED_PROFILE,
    QSYS_RES_QOS_MODE,
    QSYS_RES_CFG,
    QSYS_RES_STAT,
    QSYS_EGR_DROP_MODE,
    QSYS_EQ_CTRL,
    QSYS_EVENTS_CORE,
    QSYS_QMAXSDU_CFG_0,
    QSYS_QMAXSDU_CFG_1,
    QSYS_QMAXSDU_CFG_2,
    QSYS_QMAXSDU_CFG_3,
    QSYS_QMAXSDU_CFG_4,
    QSYS_QMAXSDU_CFG_5,
    QSYS_QMAXSDU_CFG_6,
    QSYS_QMAXSDU_CFG_7,
    QSYS_PREEMPTION_CFG,
    QSYS_CIR_CFG,
    QSYS_EIR_CFG,
    QSYS_SE_CFG,
    QSYS_SE_DWRR_CFG,
    QSYS_SE_CONNECT,
    QSYS_SE_DLB_SENSE,
    QSYS_CIR_STATE,
    QSYS_EIR_STATE,
    QSYS_SE_STATE,
    QSYS_HSCH_MISC_CFG,
    QSYS_TAG_CONFIG,
    QSYS_TAS_PARAM_CFG_CTRL,
    QSYS_PORT_MAX_SDU,
    QSYS_PARAM_CFG_REG_1,
    QSYS_PARAM_CFG_REG_2,
    QSYS_PARAM_CFG_REG_3,
    QSYS_PARAM_CFG_REG_4,
    QSYS_PARAM_CFG_REG_5,
    QSYS_GCL_CFG_REG_1,
    QSYS_GCL_CFG_REG_2,
    QSYS_PARAM_STATUS_REG_1,
    QSYS_PARAM_STATUS_REG_2,
    QSYS_PARAM_STATUS_REG_3,
    QSYS_PARAM_STATUS_REG_4,
    QSYS_PARAM_STATUS_REG_5,
    QSYS_PARAM_STATUS_REG_6,
    QSYS_PARAM_STATUS_REG_7,
    QSYS_PARAM_STATUS_REG_8,
    QSYS_PARAM_STATUS_REG_9,
    QSYS_GCL_STATUS_REG_1,
    QSYS_GCL_STATUS_REG_2,
    REW_PORT_VLAN_CFG = REW << TARGET_OFFSET,
    REW_TAG_CFG,
    REW_PORT_CFG,
    REW_DSCP_CFG,
    REW_PCP_DEI_QOS_MAP_CFG,
    REW_PTP_CFG,
    REW_PTP_DLY1_CFG,
    REW_RED_TAG_CFG,
    REW_DSCP_REMAP_DP1_CFG,
    REW_DSCP_REMAP_CFG,
    REW_STAT_CFG,
    REW_REW_STICKY,
    REW_PPT,
    SYS_COUNT_RX_OCTETS = SYS << TARGET_OFFSET,
    SYS_COUNT_RX_UNICAST,
    SYS_COUNT_RX_MULTICAST,
    SYS_COUNT_RX_BROADCAST,
    SYS_COUNT_RX_SHORTS,
    SYS_COUNT_RX_FRAGMENTS,
    SYS_COUNT_RX_JABBERS,
    SYS_COUNT_RX_CRC_ALIGN_ERRS,
    SYS_COUNT_RX_SYM_ERRS,
    SYS_COUNT_RX_64,
    SYS_COUNT_RX_65_127,
    SYS_COUNT_RX_128_255,
    SYS_COUNT_RX_256_511,
    SYS_COUNT_RX_512_1023,
    SYS_COUNT_RX_1024_1526,
    SYS_COUNT_RX_1527_MAX,
    SYS_COUNT_RX_PAUSE,
    SYS_COUNT_RX_CONTROL,
    SYS_COUNT_RX_LONGS,
    SYS_COUNT_RX_CLASSIFIED_DROPS,
    SYS_COUNT_RX_RED_PRIO_0,
    SYS_COUNT_RX_RED_PRIO_1,
    SYS_COUNT_RX_RED_PRIO_2,
    SYS_COUNT_RX_RED_PRIO_3,
    SYS_COUNT_RX_RED_PRIO_4,
    SYS_COUNT_RX_RED_PRIO_5,
    SYS_COUNT_RX_RED_PRIO_6,
    SYS_COUNT_RX_RED_PRIO_7,
    SYS_COUNT_RX_YELLOW_PRIO_0,
    SYS_COUNT_RX_YELLOW_PRIO_1,
    SYS_COUNT_RX_YELLOW_PRIO_2,
    SYS_COUNT_RX_YELLOW_PRIO_3,
    SYS_COUNT_RX_YELLOW_PRIO_4,
    SYS_COUNT_RX_YELLOW_PRIO_5,
    SYS_COUNT_RX_YELLOW_PRIO_6,
    SYS_COUNT_RX_YELLOW_PRIO_7,
    SYS_COUNT_RX_GREEN_PRIO_0,
    SYS_COUNT_RX_GREEN_PRIO_1,
    SYS_COUNT_RX_GREEN_PRIO_2,
    SYS_COUNT_RX_GREEN_PRIO_3,
    SYS_COUNT_RX_GREEN_PRIO_4,
    SYS_COUNT_RX_GREEN_PRIO_5,
    SYS_COUNT_RX_GREEN_PRIO_6,
    SYS_COUNT_RX_GREEN_PRIO_7,
    SYS_COUNT_RX_ASSEMBLY_ERRS,
    SYS_COUNT_RX_SMD_ERRS,
    SYS_COUNT_RX_ASSEMBLY_OK,
    SYS_COUNT_RX_MERGE_FRAGMENTS,
    SYS_COUNT_RX_PMAC_OCTETS,
    SYS_COUNT_RX_PMAC_UNICAST,
    SYS_COUNT_RX_PMAC_MULTICAST,
    SYS_COUNT_RX_PMAC_BROADCAST,
    SYS_COUNT_RX_PMAC_SHORTS,
    SYS_COUNT_RX_PMAC_FRAGMENTS,
    SYS_COUNT_RX_PMAC_JABBERS,
    SYS_COUNT_RX_PMAC_CRC_ALIGN_ERRS,
    SYS_COUNT_RX_PMAC_SYM_ERRS,
    SYS_COUNT_RX_PMAC_64,
    SYS_COUNT_RX_PMAC_65_127,
    SYS_COUNT_RX_PMAC_128_255,
    SYS_COUNT_RX_PMAC_256_511,
    SYS_COUNT_RX_PMAC_512_1023,
    SYS_COUNT_RX_PMAC_1024_1526,
    SYS_COUNT_RX_PMAC_1527_MAX,
    SYS_COUNT_RX_PMAC_PAUSE,
    SYS_COUNT_RX_PMAC_CONTROL,
    SYS_COUNT_RX_PMAC_LONGS,
    SYS_COUNT_TX_OCTETS,
    SYS_COUNT_TX_UNICAST,
    SYS_COUNT_TX_MULTICAST,
    SYS_COUNT_TX_BROADCAST,
    SYS_COUNT_TX_COLLISION,
    SYS_COUNT_TX_DROPS,
    SYS_COUNT_TX_PAUSE,
    SYS_COUNT_TX_64,
    SYS_COUNT_TX_65_127,
    SYS_COUNT_TX_128_255,
    SYS_COUNT_TX_256_511,
    SYS_COUNT_TX_512_1023,
    SYS_COUNT_TX_1024_1526,
    SYS_COUNT_TX_1527_MAX,
    SYS_COUNT_TX_YELLOW_PRIO_0,
    SYS_COUNT_TX_YELLOW_PRIO_1,
    SYS_COUNT_TX_YELLOW_PRIO_2,
    SYS_COUNT_TX_YELLOW_PRIO_3,
    SYS_COUNT_TX_YELLOW_PRIO_4,
    SYS_COUNT_TX_YELLOW_PRIO_5,
    SYS_COUNT_TX_YELLOW_PRIO_6,
    SYS_COUNT_TX_YELLOW_PRIO_7,
    SYS_COUNT_TX_GREEN_PRIO_0,
    SYS_COUNT_TX_GREEN_PRIO_1,
    SYS_COUNT_TX_GREEN_PRIO_2,
    SYS_COUNT_TX_GREEN_PRIO_3,
    SYS_COUNT_TX_GREEN_PRIO_4,
    SYS_COUNT_TX_GREEN_PRIO_5,
    SYS_COUNT_TX_GREEN_PRIO_6,
    SYS_COUNT_TX_GREEN_PRIO_7,
    SYS_COUNT_TX_AGED,
    SYS_COUNT_TX_MM_HOLD,
    SYS_COUNT_TX_MERGE_FRAGMENTS,
    SYS_COUNT_TX_PMAC_OCTETS,
    SYS_COUNT_TX_PMAC_UNICAST,
    SYS_COUNT_TX_PMAC_MULTICAST,
    SYS_COUNT_TX_PMAC_BROADCAST,
    SYS_COUNT_TX_PMAC_PAUSE,
    SYS_COUNT_TX_PMAC_64,
    SYS_COUNT_TX_PMAC_65_127,
    SYS_COUNT_TX_PMAC_128_255,
    SYS_COUNT_TX_PMAC_256_511,
    SYS_COUNT_TX_PMAC_512_1023,
    SYS_COUNT_TX_PMAC_1024_1526,
    SYS_COUNT_TX_PMAC_1527_MAX,
    SYS_COUNT_DROP_LOCAL,
    SYS_COUNT_DROP_TAIL,
    SYS_COUNT_DROP_YELLOW_PRIO_0,
    SYS_COUNT_DROP_YELLOW_PRIO_1,
    SYS_COUNT_DROP_YELLOW_PRIO_2,
    SYS_COUNT_DROP_YELLOW_PRIO_3,
    SYS_COUNT_DROP_YELLOW_PRIO_4,
    SYS_COUNT_DROP_YELLOW_PRIO_5,
    SYS_COUNT_DROP_YELLOW_PRIO_6,
    SYS_COUNT_DROP_YELLOW_PRIO_7,
    SYS_COUNT_DROP_GREEN_PRIO_0,
    SYS_COUNT_DROP_GREEN_PRIO_1,
    SYS_COUNT_DROP_GREEN_PRIO_2,
    SYS_COUNT_DROP_GREEN_PRIO_3,
    SYS_COUNT_DROP_GREEN_PRIO_4,
    SYS_COUNT_DROP_GREEN_PRIO_5,
    SYS_COUNT_DROP_GREEN_PRIO_6,
    SYS_COUNT_DROP_GREEN_PRIO_7,
    SYS_COUNT_SF_MATCHING_FRAMES,
    SYS_COUNT_SF_NOT_PASSING_FRAMES,
    SYS_COUNT_SF_NOT_PASSING_SDU,
    SYS_COUNT_SF_RED_FRAMES,
    SYS_RESET_CFG,
    SYS_SR_ETYPE_CFG,
    SYS_VLAN_ETYPE_CFG,
    SYS_PORT_MODE,
    SYS_FRONT_PORT_MODE,
    SYS_FRM_AGING,
    SYS_STAT_CFG,
    SYS_SW_STATUS,
    SYS_MISC_CFG,
    SYS_REW_MAC_HIGH_CFG,
    SYS_REW_MAC_LOW_CFG,
    SYS_TIMESTAMP_OFFSET,
    SYS_CMID,
    SYS_PAUSE_CFG,
    SYS_PAUSE_TOT_CFG,
    SYS_ATOP,
    SYS_ATOP_TOT_CFG,
    SYS_MAC_FC_CFG,
    SYS_MMGT,
    SYS_MMGT_FAST,
    SYS_EVENTS_DIF,
    SYS_EVENTS_CORE,
    SYS_PTP_STATUS,
    SYS_PTP_TXSTAMP,
    SYS_PTP_NXT,
    SYS_PTP_CFG,
    SYS_RAM_INIT,
    SYS_CM_ADDR,
    SYS_CM_DATA_WR,
    SYS_CM_DATA_RD,
    SYS_CM_OP,
    SYS_CM_DATA,
    PTP_PIN_CFG = PTP << TARGET_OFFSET,
    PTP_PIN_TOD_SEC_MSB,
    PTP_PIN_TOD_SEC_LSB,
    PTP_PIN_TOD_NSEC,
    PTP_PIN_WF_HIGH_PERIOD,
    PTP_PIN_WF_LOW_PERIOD,
    PTP_CFG_MISC,
    PTP_CLK_CFG_ADJ_CFG,
    PTP_CLK_CFG_ADJ_FREQ,
    GCB_SOFT_RST = GCB << TARGET_OFFSET,
    GCB_MIIM_MII_STATUS,
    GCB_MIIM_MII_CMD,
    GCB_MIIM_MII_DATA,
    DEV_CLOCK_CFG = DEV_GMII << TARGET_OFFSET,
    DEV_PORT_MISC,
    DEV_EVENTS,
    DEV_EEE_CFG,
    DEV_RX_PATH_DELAY,
    DEV_TX_PATH_DELAY,
    DEV_PTP_PREDICT_CFG,
    DEV_MAC_ENA_CFG,
    DEV_MAC_MODE_CFG,
    DEV_MAC_MAXLEN_CFG,
    DEV_MAC_TAGS_CFG,
    DEV_MAC_ADV_CHK_CFG,
    DEV_MAC_IFG_CFG,
    DEV_MAC_HDX_CFG,
    DEV_MAC_DBG_CFG,
    DEV_MAC_FC_MAC_LOW_CFG,
    DEV_MAC_FC_MAC_HIGH_CFG,
    DEV_MAC_STICKY,
    DEV_MM_ENABLE_CONFIG,
    DEV_MM_VERIF_CONFIG,
    DEV_MM_STATUS,
    PCS1G_CFG,
    PCS1G_MODE_CFG,
    PCS1G_SD_CFG,
    PCS1G_ANEG_CFG,
    PCS1G_ANEG_NP_CFG,
    PCS1G_LB_CFG,
    PCS1G_DBG_CFG,
    PCS1G_CDET_CFG,
    PCS1G_ANEG_STATUS,
    PCS1G_ANEG_NP_STATUS,
    PCS1G_LINK_STATUS,
    PCS1G_LINK_DOWN_CNT,
    PCS1G_STICKY,
    PCS1G_DEBUG_STATUS,
    PCS1G_LPI_CFG,
    PCS1G_LPI_WAKE_ERROR_CNT,
    PCS1G_LPI_STATUS,
    PCS1G_TSTPAT_MODE_CFG,
    PCS1G_TSTPAT_STATUS,
    DEV_PCS_FX100_CFG,
    DEV_PCS_FX100_STATUS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocelot_regfield {
    ANA_ADVLEARN_VLAN_CHK,
    ANA_ADVLEARN_LEARN_MIRROR,
    ANA_ANEVENTS_FLOOD_DISCARD,
    ANA_ANEVENTS_MSTI_DROP,
    ANA_ANEVENTS_ACLKILL,
    ANA_ANEVENTS_ACLUSED,
    ANA_ANEVENTS_AUTOAGE,
    ANA_ANEVENTS_VS2TTL1,
    ANA_ANEVENTS_STORM_DROP,
    ANA_ANEVENTS_LEARN_DROP,
    ANA_ANEVENTS_AGED_ENTRY,
    ANA_ANEVENTS_CPU_LEARN_FAILED,
    ANA_ANEVENTS_AUTO_LEARN_FAILED,
    ANA_ANEVENTS_LEARN_REMOVE,
    ANA_ANEVENTS_AUTO_LEARNED,
    ANA_ANEVENTS_AUTO_MOVED,
    ANA_ANEVENTS_DROPPED,
    ANA_ANEVENTS_CLASSIFIED_DROP,
    ANA_ANEVENTS_CLASSIFIED_COPY,
    ANA_ANEVENTS_VLAN_DISCARD,
    ANA_ANEVENTS_FWD_DISCARD,
    ANA_ANEVENTS_MULTICAST_FLOOD,
    ANA_ANEVENTS_UNICAST_FLOOD,
    ANA_ANEVENTS_DEST_KNOWN,
    ANA_ANEVENTS_BUCKET3_MATCH,
    ANA_ANEVENTS_BUCKET2_MATCH,
    ANA_ANEVENTS_BUCKET1_MATCH,
    ANA_ANEVENTS_BUCKET0_MATCH,
    ANA_ANEVENTS_CPU_OPERATION,
    ANA_ANEVENTS_DMAC_LOOKUP,
    ANA_ANEVENTS_SMAC_LOOKUP,
    ANA_ANEVENTS_SEQ_GEN_ERR_0,
    ANA_ANEVENTS_SEQ_GEN_ERR_1,
    ANA_TABLES_MACACCESS_B_DOM,
    ANA_TABLES_MACTINDX_BUCKET,
    ANA_TABLES_MACTINDX_M_INDEX,
    QSYS_SWITCH_PORT_MODE_PORT_ENA,
    QSYS_SWITCH_PORT_MODE_SCH_NEXT_CFG,
    QSYS_SWITCH_PORT_MODE_YEL_RSRVD,
    QSYS_SWITCH_PORT_MODE_INGRESS_DROP_MODE,
    QSYS_SWITCH_PORT_MODE_TX_PFC_ENA,
    QSYS_SWITCH_PORT_MODE_TX_PFC_MODE,
    QSYS_TIMED_FRAME_ENTRY_TFRM_VLD,
    QSYS_TIMED_FRAME_ENTRY_TFRM_FP,
    QSYS_TIMED_FRAME_ENTRY_TFRM_PORTNO,
    QSYS_TIMED_FRAME_ENTRY_TFRM_TM_SEL,
    QSYS_TIMED_FRAME_ENTRY_TFRM_TM_T,
    SYS_PORT_MODE_DATA_WO_TS,
    SYS_PORT_MODE_INCL_INJ_HDR,
    SYS_PORT_MODE_INCL_XTR_HDR,
    SYS_PORT_MODE_INCL_HDR_ERR,
    SYS_RESET_CFG_CORE_ENA,
    SYS_RESET_CFG_MEM_ENA,
    SYS_RESET_CFG_MEM_INIT,
    GCB_SOFT_RST_SWC_RST,
    GCB_MIIM_MII_STATUS_PENDING,
    GCB_MIIM_MII_STATUS_BUSY,
    SYS_PAUSE_CFG_PAUSE_START,
    SYS_PAUSE_CFG_PAUSE_STOP,
    SYS_PAUSE_CFG_PAUSE_ENA,
    REGFIELD_MAX
}

// VCAP_CORE_CFG
// VCAP_CORE_CACHE
// VCAP_CONST
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocelot_ptp_pins {
    PTP_PIN_0,
    PTP_PIN_1,
    PTP_PIN_2,
    PTP_PIN_3,
    TOD_ACC_PIN
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocelot_tag_prefix {
    OCELOT_TAG_PREFIX_DISABLED	= 0,
    OCELOT_TAG_PREFIX_NONE,
    OCELOT_TAG_PREFIX_SHORT,
    OCELOT_TAG_PREFIX_LONG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_ops {
    pub port): *mut *mut *mut *mut net_device (port_to_netdev)(ocelot ocelot, int,
    pub dev): *mut *mut *mut int (netdev_to_port)(struct ocelot ocelot, struct net_device,
    pub ocelot): *mut *mut int (reset)(struct ocelot,
    pub value): *mut *mut u16 (wm_enc)(u16,
    pub value): *mut *mut u16 (wm_dec)(u16,
    pub maxuse): *mut *mut *mut void (wm_stat)(u32 val, u32 inuse, u32,
    pub ocelot): *mut *mut void (psfp_init)(struct ocelot,
    pub f): *mut flow_cls_offload,
    pub f): *mut *mut *mut int (psfp_filter_del)(struct ocelot ocelot, struct flow_cls_offload,
    pub stats): *mut flow_stats,
    pub ocelot): *mut *mut void (cut_through_fwd)(struct ocelot,
    pub ocelot): *mut *mut void (tas_clock_adjust)(struct ocelot,
    pub port): *mut *mut *mut void (tas_guard_bands_update)(struct ocelot ocelot, int,
    pub ocelot): *mut *mut void (update_stats)(struct ocelot,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_vcap_policer {
    pub pol_list: list_head,
    pub base: u16,
    pub max: u16,
    pub base2: u16,
    pub max2: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_vcap_block {
    pub rules: list_head,
    pub count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_bridge_vlan {
    pub vid: u16,
    pub portmask: c_ulong,
    pub untagged: c_ulong,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocelot_port_tag_config {
// all VLANs are egress-untagged
    OCELOT_PORT_TAG_DISABLED = 0,
// all VLANs except the native VLAN and VID 0 are egress-tagged
    OCELOT_PORT_TAG_NATIVE = 1,
// all VLANs except VID 0 are egress-tagged
    OCELOT_PORT_TAG_TRUNK_NO_VID0 = 2,
// all VLANs are egress-tagged
    OCELOT_PORT_TAG_TRUNK = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_psfp_list {
    pub stream_list: list_head,
    pub sfi_list: list_head,
    pub sgi_list: list_head,
// Serialize access to the lists
    pub lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocelot_sb {
    OCELOT_SB_BUF,
    OCELOT_SB_REF,
    OCELOT_SB_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocelot_sb_pool {
    OCELOT_SB_POOL_ING,
    OCELOT_SB_POOL_EGR,
    OCELOT_SB_POOL_NUM,
}

// MAC table entry types.
// ENTRYTYPE_NORMAL is subject to aging.
// ENTRYTYPE_LOCKED is not subject to aging.
// ENTRYTYPE_MACv4 is not subject to aging. For IPv4 multicast.
// ENTRYTYPE_MACv6 is not subject to aging. For IPv6 multicast.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum macaccess_entry_type {
    ENTRYTYPE_NORMAL = 0,
    ENTRYTYPE_LOCKED,
    ENTRYTYPE_MACv4,
    ENTRYTYPE_MACv6,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocelot_proto {
    OCELOT_PROTO_PTP_L2 = BIT(0),
    OCELOT_PROTO_PTP_L4 = BIT(1),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_lag_fdb {
    pub addr: [c_uchar; ETH_ALEN],
    pub vid: u16,
    pub bond: *mut net_device,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_mirror {
    pub refcount: refcount_t,
    pub to: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_mm_state {
    pub verify_status: ethtool_mm_verify_status,
    pub tx_enabled: bool,
    pub tx_active: bool,
    pub preemptible_tcs: u8,
    pub active_preemptible_tcs: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_ts_stats {
    pub pkts: u64,
    pub onestep_pkts_unconfirmed: u64,
    pub lost: u64,
    pub err: u64,
    pub syncp: u64_stats_sync,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_port {
    pub ocelot: *mut ocelot,
    pub target: *mut regmap,
    pub bond: *mut net_device,
    pub bridge: *mut net_device,
    pub dsa_8021q_cpu: *mut ocelot_port,
// VLAN that untagged frames are classified to, on ingress
    pub pvid_vlan: *const ocelot_bridge_vlan,
    pub taprio: *mut tc_taprio_qopt_offload,
    pub phy_mode: phy_interface_t,
    pub ts_stats: *mut ocelot_ts_stats,
    pub tx_skbs: sk_buff_head,
    pub trap_proto: c_uint,
    pub mrp_ring_id: u16,
    pub ptp_cmd: u8,
    pub index: u8,
    pub stp_state: u8,
    pub vlan_aware: bool,
    pub is_dsa_8021q_cpu: bool,
    pub learn_ena: bool,
    pub lag_tx_active: bool,
    pub bridge_num: c_int,
    pub speed: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot {
    pub dev: *mut device,
    pub devlink: *mut devlink,
    pub devlink_ports: *mut devlink_port,
    pub ops: *const ocelot_ops,
    pub targets: [*mut regmap; TARGET_MAX],
    pub regfields: [*mut regmap_field; REGFIELD_MAX],
    pub map: *const *const u32,
    pub stats_regions: list_head,
    pub inj_lock: spinlock_t,
    pub xtr_lock: spinlock_t,
    pub pool_size: [u32; OCELOT_SB_NUM][OCELOT_SB_POOL_NUM],
    pub packet_buffer_size: c_int,
    pub num_frame_refs: c_int,
    pub num_mact_rows: c_int,
    pub ports: *mut ocelot_port,
    pub base_mac: [u8; ETH_ALEN],
    pub vlans: list_head,
    pub traps: list_head,
    pub lag_fdbs: list_head,
// Switches like VSC9959 have flooding per traffic class
    pub num_flooding_pgids: c_int,
// In tables like ANA:PORT and the ANA:PGID:PGID mask,
// the CPU is located after the physical ports (at the
// num_phys_ports index).
//
    pub num_phys_ports: u8,
    pub npi: c_int,
    pub npi_inj_prefix: ocelot_tag_prefix,
    pub npi_xtr_prefix: ocelot_tag_prefix,
    pub bridges: c_ulong,
    pub multicast: list_head,
    pub pgids: list_head,
    pub dummy_rules: list_head,
    pub block: [ocelot_vcap_block; 3],
    pub vcap_pol: ocelot_vcap_policer,
    pub vcap: *mut vcap_props,
    pub mirror: *mut ocelot_mirror,
    pub psfp: ocelot_psfp_list,
// Workqueue to check statistics for overflow
    pub stats_work: delayed_work,
    pub stats_queue: *mut workqueue_struct,
// Lock for serializing access to the statistics array
    pub stats_lock: spinlock_t,
    pub stats: *mut u64,
// Lock for serializing indirect access to STAT_VIEW registers
    pub stat_view_lock: mutex,
// Lock for serializing access to the MAC table
    pub mact_lock: mutex,
// Lock for serializing forwarding domain changes, including the
// configuration of the Time-Aware Shaper, MAC Merge layer and
// cut-through forwarding, on which it depends
//
    pub fwd_domain_lock: mutex,
    pub owq: *mut workqueue_struct,
    pub ptp:1: u8,
    pub mm_supported:1: u8,
    pub ptp_clock: *mut ptp_clock,
    pub ptp_info: ptp_clock_info,
    pub ptp_skbs_in_flight: c_uint,
// Protects the 2-step TX timestamp ID logic
    pub ts_id_lock: spinlock_t,
// Protects the PTP clock
    pub ptp_clock_lock: spinlock_t,
    pub ptp_pins: [ptp_pin_desc; OCELOT_PTP_PINS_NUM],
    pub mm: *mut ocelot_mm_state,
    pub fdma: *mut ocelot_fdma,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_policer {
    pub /: *mut *mut u32 rate; / kilobit per second,
    pub /: *mut *mut u32 burst; / bytes,
}

// I/O
extern "C" {
    pub fn ocelot_port_readl(port: *mut ocelot_port, reg: ocelot_reg) -> u32;
}
extern "C" {
    pub fn ocelot_port_writel(port: *mut ocelot_port, val: u32, reg: ocelot_reg);
}
extern "C" {
    pub fn __ocelot_read_ix(ocelot: *mut ocelot, reg: ocelot_reg, offset: u32) -> u32;
}
// Packet I/O
extern "C" {
    pub fn ocelot_lock_inj_grp(ocelot: *mut ocelot, grp: c_int);
}
extern "C" {
    pub fn ocelot_unlock_inj_grp(ocelot: *mut ocelot, grp: c_int);
}
extern "C" {
    pub fn ocelot_lock_xtr_grp(ocelot: *mut ocelot, grp: c_int);
}
extern "C" {
    pub fn ocelot_unlock_xtr_grp(ocelot: *mut ocelot, grp: c_int);
}
extern "C" {
    pub fn ocelot_lock_xtr_grp_bh(ocelot: *mut ocelot, grp: c_int);
}
extern "C" {
    pub fn ocelot_unlock_xtr_grp_bh(ocelot: *mut ocelot, grp: c_int);
}
extern "C" {
    pub fn ocelot_can_inject(ocelot: *mut ocelot, grp: c_int) -> bool;
}
extern "C" {
    pub fn ocelot_xtr_poll_frame(ocelot: *mut ocelot, grp: c_int, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn ocelot_drain_cpu_queue(ocelot: *mut ocelot, grp: c_int);
}
// Hardware initialization
extern "C" {
    pub fn ocelot_reset(ocelot: *mut ocelot) -> c_int;
}
extern "C" {
    pub fn ocelot_init(ocelot: *mut ocelot) -> c_int;
}
extern "C" {
    pub fn ocelot_deinit(ocelot: *mut ocelot);
}
extern "C" {
    pub fn ocelot_init_port(ocelot: *mut ocelot, port: c_int);
}
extern "C" {
    pub fn ocelot_deinit_port(ocelot: *mut ocelot, port: c_int);
}
extern "C" {
    pub fn ocelot_port_setup_dsa_8021q_cpu(ocelot: *mut ocelot, cpu: c_int);
}
extern "C" {
    pub fn ocelot_port_teardown_dsa_8021q_cpu(ocelot: *mut ocelot, cpu: c_int);
}
extern "C" {
    pub fn ocelot_port_assign_dsa_8021q_cpu(ocelot: *mut ocelot, port: c_int, cpu: c_int);
}
extern "C" {
    pub fn ocelot_port_unassign_dsa_8021q_cpu(ocelot: *mut ocelot, port: c_int);
}
extern "C" {
    pub fn ocelot_port_assigned_dsa_8021q_cpu_mask(ocelot: *mut ocelot, port: c_int) -> u32;
}
// Watermark interface
extern "C" {
    pub fn ocelot_wm_enc(value: u16) -> u16;
}
extern "C" {
    pub fn ocelot_wm_dec(wm: u16) -> u16;
}
extern "C" {
    pub fn ocelot_wm_stat(val: u32, inuse: *mut u32, maxuse: *mut u32);
}
// DSA callbacks
extern "C" {
    pub fn ocelot_get_strings(ocelot: *mut ocelot, port: c_int, sset: u32, data: *mut u8);
}
extern "C" {
    pub fn ocelot_get_ethtool_stats(ocelot: *mut ocelot, port: c_int, data: *mut u64);
}
extern "C" {
    pub fn ocelot_get_sset_count(ocelot: *mut ocelot, port: c_int, sset: c_int) -> c_int;
}
extern "C" {
    pub fn ocelot_set_ageing_time(ocelot: *mut ocelot, msecs: c_uint);
}
extern "C" {
    pub fn ocelot_bridge_stp_state_set(ocelot: *mut ocelot, port: c_int, state: u8);
}
extern "C" {
    pub fn ocelot_get_bridge_fwd_mask(ocelot: *mut ocelot, src_port: c_int) -> u32;
}
extern "C" {
    pub fn ocelot_port_get_default_prio(ocelot: *mut ocelot, port: c_int) -> c_int;
}
extern "C" {
    pub fn ocelot_port_set_default_prio(ocelot: *mut ocelot, port: c_int, prio: u8) -> c_int;
}
extern "C" {
    pub fn ocelot_port_get_dscp_prio(ocelot: *mut ocelot, port: c_int, dscp: u8) -> c_int;
}
extern "C" {
    pub fn ocelot_port_add_dscp_prio(ocelot: *mut ocelot, port: c_int, dscp: u8, prio: u8) -> c_int;
}
extern "C" {
    pub fn ocelot_port_del_dscp_prio(ocelot: *mut ocelot, port: c_int, dscp: u8, prio: u8) -> c_int;
}
extern "C" {
    pub fn ocelot_mact_flush(ocelot: *mut ocelot, port: c_int) -> c_int;
}
extern "C" {
    pub fn ocelot_vlan_del(ocelot: *mut ocelot, port: c_int, vid: u16) -> c_int;
}
extern "C" {
    pub fn ocelot_get_txtstamp(ocelot: *mut ocelot);
}
extern "C" {
    pub fn ocelot_port_set_maxlen(ocelot: *mut ocelot, port: c_int, sdu: usize);
}
extern "C" {
    pub fn ocelot_get_max_mtu(ocelot: *mut ocelot, port: c_int) -> c_int;
}
extern "C" {
    pub fn ocelot_port_policer_del(ocelot: *mut ocelot, port: c_int) -> c_int;
}
extern "C" {
    pub fn ocelot_port_mirror_del(ocelot: *mut ocelot, from: c_int, ingress: bool);
}
extern "C" {
    pub fn ocelot_port_lag_change(ocelot: *mut ocelot, port: c_int, lag_tx_active: bool);
}
extern "C" {
    pub fn ocelot_bond_get_id(ocelot: *mut ocelot, bond: *mut net_device) -> c_int;
}
extern "C" {
    pub fn ocelot_devlink_sb_register(ocelot: *mut ocelot) -> c_int;
}
extern "C" {
    pub fn ocelot_devlink_sb_unregister(ocelot: *mut ocelot);
}
extern "C" {
    pub fn ocelot_sb_occ_snapshot(ocelot: *mut ocelot, sb_index: c_uint) -> c_int;
}
extern "C" {
    pub fn ocelot_sb_occ_max_clear(ocelot: *mut ocelot, sb_index: c_uint) -> c_int;
}
extern "C" {
    pub fn ocelot_vcap_policer_del(ocelot: *mut ocelot, pol_ix: u32) -> c_int;
}
extern "C" {
    pub fn ocelot_mm_irq(ocelot: *mut ocelot);
}

extern "C" {
    pub fn ocelot_pll5_init(ocelot: *mut ocelot);
}
