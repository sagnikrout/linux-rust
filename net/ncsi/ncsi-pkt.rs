//! Automatically rewritten from C Header to Rust Module
//! Source: net/ncsi/ncsi-pkt.h
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
// Copyright Gavin Shan, IBM Corporation 2016.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_pkt_hdr {
    pub /: *mut *mut unsigned char mc_id; / Management controller ID,
    pub /: *mut *mut unsigned char revision; / NCSI version - 0x01,
    pub /: *mut *mut unsigned char reserved; / Reserved,
    pub /: *mut *mut unsigned char id; / Packet sequence number,
    pub /: *mut *mut unsigned char type; / Packet type,
    pub /: *mut *mut unsigned char channel; / Network controller ID,
    pub /: *mut *mut __be16 length; / Payload length,
    pub /: *mut *mut __be32 reserved1[2]; / Reserved,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_cmd_pkt_hdr {
    pub /: *mut *mut ncsi_pkt_hdr common; / Common NCSI packet header,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_rsp_pkt_hdr {
    pub /: *mut *mut ncsi_pkt_hdr common; / Common NCSI packet header,
    pub /: *mut *mut __be16 code; / Response code,
    pub /: *mut *mut __be16 reason; / Response reason,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_aen_pkt_hdr {
    pub /: *mut *mut ncsi_pkt_hdr common; / Common NCSI packet header,
    pub /: *mut *mut unsigned char reserved2[3]; / Reserved,
    pub /: *mut *mut unsigned char type; / AEN packet type,
}

// NCSI common command packet
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_cmd_pkt {
    pub /: *mut *mut ncsi_cmd_pkt_hdr cmd; / Command header,
    pub /: *mut *mut __be32 checksum; / Checksum,
    pub pad: [c_uchar; 26],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_rsp_pkt {
    pub /: *mut *mut ncsi_rsp_pkt_hdr rsp; / Response header,
    pub /: *mut *mut __be32 checksum; / Checksum,
    pub pad: [c_uchar; 22],
}

// Select Package
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_cmd_sp_pkt {
    pub /: *mut *mut ncsi_cmd_pkt_hdr cmd; / Command header,
    pub /: *mut *mut unsigned char reserved[3]; / Reserved,
    pub /: *mut *mut unsigned char hw_arbitration; / HW arbitration,
    pub /: *mut *mut __be32 checksum; / Checksum,
    pub pad: [c_uchar; 22],
}

// Disable Channel
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_cmd_dc_pkt {
    pub /: *mut *mut ncsi_cmd_pkt_hdr cmd; / Command header,
    pub /: *mut *mut unsigned char reserved[3]; / Reserved,
    pub /: *mut *mut unsigned char ald; / Allow link down,
    pub /: *mut *mut __be32 checksum; / Checksum,
    pub pad: [c_uchar; 22],
}

// Reset Channel
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_cmd_rc_pkt {
    pub /: *mut *mut ncsi_cmd_pkt_hdr cmd; / Command header,
    pub /: *mut *mut __be32 reserved; / Reserved,
    pub /: *mut *mut __be32 checksum; / Checksum,
    pub pad: [c_uchar; 22],
}

// AEN Enable
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_cmd_ae_pkt {
    pub /: *mut *mut ncsi_cmd_pkt_hdr cmd; / Command header,
    pub /: *mut *mut unsigned char reserved[3]; / Reserved,
    pub /: *mut *mut unsigned char mc_id; / MC ID,
    pub /: *mut *mut __be32 mode; / AEN working mode,
    pub /: *mut *mut __be32 checksum; / Checksum,
    pub pad: [c_uchar; 18],
}

// Set Link
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_cmd_sl_pkt {
    pub /: *mut *mut ncsi_cmd_pkt_hdr cmd; / Command header,
    pub /: *mut *mut __be32 mode; / Link working mode,
    pub /: *mut *mut __be32 oem_mode; / OEM link mode,
    pub /: *mut *mut __be32 checksum; / Checksum,
    pub pad: [c_uchar; 18],
}

// Set VLAN Filter
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_cmd_svf_pkt {
    pub /: *mut *mut ncsi_cmd_pkt_hdr cmd; / Command header,
    pub /: *mut *mut __be16 reserved; / Reserved,
    pub /: *mut *mut __be16 vlan; / VLAN ID,
    pub /: *mut *mut __be16 reserved1; / Reserved,
    pub /: *mut *mut unsigned char index; / VLAN table index,
    pub /: *mut *mut unsigned char enable; / Enable or disable,
    pub /: *mut *mut __be32 checksum; / Checksum,
    pub pad: [c_uchar; 18],
}

// Enable VLAN
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_cmd_ev_pkt {
    pub /: *mut *mut ncsi_cmd_pkt_hdr cmd; / Command header,
    pub /: *mut *mut unsigned char reserved[3]; / Reserved,
    pub /: *mut *mut unsigned char mode; / VLAN filter mode,
    pub /: *mut *mut __be32 checksum; / Checksum,
    pub pad: [c_uchar; 22],
}

// Set MAC Address
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_cmd_sma_pkt {
    pub /: *mut *mut ncsi_cmd_pkt_hdr cmd; / Command header,
    pub /: *mut *mut unsigned char mac[6]; / MAC address,
    pub /: *mut *mut unsigned char index; / MAC table index,
    pub /: *mut *mut unsigned char at_e; / Addr type and operation,
    pub /: *mut *mut __be32 checksum; / Checksum,
    pub pad: [c_uchar; 18],
}

// Enable Broadcast Filter
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_cmd_ebf_pkt {
    pub /: *mut *mut ncsi_cmd_pkt_hdr cmd; / Command header,
    pub /: *mut *mut __be32 mode; / Filter mode,
    pub /: *mut *mut __be32 checksum; / Checksum,
    pub pad: [c_uchar; 22],
}

// Enable Global Multicast Filter
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_cmd_egmf_pkt {
    pub /: *mut *mut ncsi_cmd_pkt_hdr cmd; / Command header,
    pub /: *mut *mut __be32 mode; / Global MC mode,
    pub /: *mut *mut __be32 checksum; / Checksum,
    pub pad: [c_uchar; 22],
}

// Set NCSI Flow Control
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_cmd_snfc_pkt {
    pub /: *mut *mut ncsi_cmd_pkt_hdr cmd; / Command header,
    pub /: *mut *mut unsigned char reserved[3]; / Reserved,
    pub /: *mut *mut unsigned char mode; / Flow control mode,
    pub /: *mut *mut __be32 checksum; / Checksum,
    pub pad: [c_uchar; 22],
}

// OEM Request Command as per NCSI Specification
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_cmd_oem_pkt {
    pub /: *mut *mut ncsi_cmd_pkt_hdr cmd; / Command header,
    pub /: *mut *mut __be32 mfr_id; / Manufacture ID,
    pub /: *mut *mut unsigned char data[]; / OEM Payload Data,
}

// OEM Response Packet as per NCSI Specification
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_rsp_oem_pkt {
    pub /: *mut *mut ncsi_rsp_pkt_hdr rsp; / Command header,
    pub /: *mut *mut __be32 mfr_id; / Manufacture ID,
    pub /: *mut *mut unsigned char data[]; / Payload data,
}

// Mellanox Response Data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_rsp_oem_mlx_pkt {
    pub /: *mut *mut unsigned char cmd_rev; / Command Revision,
    pub /: *mut *mut unsigned char cmd; / Command ID,
    pub /: *mut *mut unsigned char param; / Parameter,
    pub /: *mut *mut unsigned char optional; / Optional data,
    pub /: *mut *mut unsigned char data[]; / Data,
}

// Broadcom Response Data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_rsp_oem_bcm_pkt {
    pub /: *mut *mut unsigned char ver; / Payload Version,
    pub /: *mut *mut unsigned char type; / OEM Command type,
    pub /: *mut *mut __be16 len; / Payload Length,
    pub /: *mut *mut unsigned char data[]; / Cmd specific Data,
}

// Intel Response Data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_rsp_oem_intel_pkt {
    pub /: *mut *mut unsigned char cmd; / OEM Command ID,
    pub /: *mut *mut unsigned char data[]; / Cmd specific Data,
}

// Get Link Status
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_rsp_gls_pkt {
    pub /: *mut *mut ncsi_rsp_pkt_hdr rsp; / Response header,
    pub /: *mut *mut __be32 status; / Link status,
    pub /: *mut *mut __be32 other; / Other indications,
    pub /: *mut *mut __be32 oem_status; / OEM link status,
    pub checksum: __be32,
    pub pad: [c_uchar; 10],
}

// Get Version ID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_rsp_gvi_pkt {
    pub /: *mut *mut ncsi_rsp_pkt_hdr rsp; / Response header,
    pub /: *mut *mut unsigned char major; / NCSI version major,
    pub /: *mut *mut unsigned char minor; / NCSI version minor,
    pub /: *mut *mut unsigned char update; / NCSI version update,
    pub /: *mut *mut unsigned char alpha1; / NCSI version alpha1,
    pub /: *mut *mut unsigned char reserved[3]; / Reserved,
    pub /: *mut *mut unsigned char alpha2; / NCSI version alpha2,
    pub /: *mut *mut unsigned char fw_name[12]; / f/w name string,
    pub /: *mut *mut __be32 fw_version; / f/w version,
    pub /: *mut *mut __be16 pci_ids[4]; / PCI IDs,
    pub /: *mut *mut __be32 mf_id; / Manufacture ID,
    pub checksum: __be32,
}

// Get Capabilities
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_rsp_gc_pkt {
    pub /: *mut *mut ncsi_rsp_pkt_hdr rsp; / Response header,
    pub /: *mut *mut __be32 cap; / Capabilities,
    pub /: *mut *mut __be32 bc_cap; / Broadcast cap,
    pub /: *mut *mut __be32 mc_cap; / Multicast cap,
    pub /: *mut *mut __be32 buf_cap; / Buffering cap,
    pub /: *mut *mut __be32 aen_cap; / AEN cap,
    pub /: *mut *mut unsigned char vlan_cnt; / VLAN filter count,
    pub /: *mut *mut unsigned char mixed_cnt; / Mix filter count,
    pub /: *mut *mut unsigned char mc_cnt; / MC filter count,
    pub /: *mut *mut unsigned char uc_cnt; / UC filter count,
    pub /: *mut *mut unsigned char reserved[2]; / Reserved,
    pub /: *mut *mut unsigned char vlan_mode; / VLAN mode,
    pub /: *mut *mut unsigned char channel_cnt; / Channel count,
    pub /: *mut *mut __be32 checksum; / Checksum,
}

// Get Parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_rsp_gp_pkt {
    pub /: *mut *mut ncsi_rsp_pkt_hdr rsp; / Response header,
    pub /: *mut *mut unsigned char mac_cnt; / Number of MAC addr,
    pub /: *mut *mut unsigned char reserved[2]; / Reserved,
    pub /: *mut *mut unsigned char mac_enable; / MAC addr enable flags,
    pub /: *mut *mut unsigned char vlan_cnt; / VLAN tag count,
    pub /: *mut *mut unsigned char reserved1; / Reserved,
    pub /: *mut *mut __be16 vlan_enable; / VLAN tag enable flags,
    pub /: *mut *mut __be32 link_mode; / Link setting,
    pub /: *mut *mut __be32 bc_mode; / BC filter mode,
    pub /: *mut *mut __be32 valid_modes; / Valid mode parameters,
    pub /: *mut *mut unsigned char vlan_mode; / VLAN mode,
    pub /: *mut *mut unsigned char fc_mode; / Flow control mode,
    pub /: *mut *mut unsigned char reserved2[2]; / Reserved,
    pub /: *mut *mut __be32 aen_mode; / AEN mode,
    pub /: *mut *mut unsigned char mac[6]; / Supported MAC addr,
    pub /: *mut *mut __be16 vlan; / Supported VLAN tags,
    pub /: *mut *mut __be32 checksum; / Checksum,
}

// Get Controller Packet Statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_rsp_gcps_pkt {
    pub /: *mut *mut ncsi_rsp_pkt_hdr rsp; / Response header,
    pub /: *mut *mut __be64 cnt; / Counter cleared,
    pub /: *mut *mut __be64 rx_bytes; / Rx bytes,
    pub /: *mut *mut __be64 tx_bytes; / Tx bytes,
    pub /: *mut *mut __be64 rx_uc_pkts; / Rx UC packets,
    pub /: *mut *mut __be64 rx_mc_pkts; / Rx MC packets,
    pub /: *mut *mut __be64 rx_bc_pkts; / Rx BC packets,
    pub /: *mut *mut __be64 tx_uc_pkts; / Tx UC packets,
    pub /: *mut *mut __be64 tx_mc_pkts; / Tx MC packets,
    pub /: *mut *mut __be64 tx_bc_pkts; / Tx BC packets,
    pub /: *mut *mut __be32 fcs_err; / FCS errors,
    pub /: *mut *mut __be32 align_err; / Alignment errors,
    pub /: *mut *mut __be32 false_carrier; / False carrier detection,
    pub /: *mut *mut __be32 runt_pkts; / Rx runt packets,
    pub /: *mut *mut __be32 jabber_pkts; / Rx jabber packets,
    pub /: *mut *mut __be32 rx_pause_xon; / Rx pause XON frames,
    pub /: *mut *mut __be32 rx_pause_xoff; / Rx XOFF frames,
    pub /: *mut *mut __be32 tx_pause_xon; / Tx XON frames,
    pub /: *mut *mut __be32 tx_pause_xoff; / Tx XOFF frames,
    pub /: *mut *mut __be32 tx_s_collision; / Single collision frames,
    pub /: *mut *mut __be32 tx_m_collision; / Multiple collision frames,
    pub /: *mut *mut __be32 l_collision; / Late collision frames,
    pub /: *mut *mut __be32 e_collision; / Excessive collision frames,
    pub /: *mut *mut __be32 rx_ctl_frames; / Rx control frames,
    pub /: *mut *mut __be32 rx_64_frames; / Rx 64-bytes frames,
    pub /: *mut *mut __be32 rx_127_frames; / Rx 65-127 bytes frames,
    pub /: *mut *mut __be32 rx_255_frames; / Rx 128-255 bytes frames,
    pub /: *mut *mut __be32 rx_511_frames; / Rx 256-511 bytes frames,
    pub /: *mut *mut __be32 rx_1023_frames; / Rx 512-1023 bytes frames,
    pub /: *mut *mut __be32 rx_1522_frames; / Rx 1024-1522 bytes frames,
    pub /: *mut *mut __be32 rx_9022_frames; / Rx 1523-9022 bytes frames,
    pub /: *mut *mut __be32 tx_64_frames; / Tx 64-bytes frames,
    pub /: *mut *mut __be32 tx_127_frames; / Tx 65-127 bytes frames,
    pub /: *mut *mut __be32 tx_255_frames; / Tx 128-255 bytes frames,
    pub /: *mut *mut __be32 tx_511_frames; / Tx 256-511 bytes frames,
    pub /: *mut *mut __be32 tx_1023_frames; / Tx 512-1023 bytes frames,
    pub /: *mut *mut __be32 tx_1522_frames; / Tx 1024-1522 bytes frames,
    pub /: *mut *mut __be32 tx_9022_frames; / Tx 1523-9022 bytes frames,
    pub /: *mut *mut __be64 rx_valid_bytes; / Rx valid bytes,
    pub /: *mut *mut __be32 rx_runt_pkts; / Rx error runt packets,
    pub /: *mut *mut __be32 rx_jabber_pkts; / Rx error jabber packets,
    pub /: *mut *mut __be32 checksum; / Checksum,
    pub __aligned(4): } __packed,
// Get NCSI Statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_rsp_gns_pkt {
    pub /: *mut *mut ncsi_rsp_pkt_hdr rsp; / Response header,
    pub /: *mut *mut __be32 rx_cmds; / Rx NCSI commands,
    pub /: *mut *mut __be32 dropped_cmds; / Dropped commands,
    pub /: *mut *mut __be32 cmd_type_errs; / Command type errors,
    pub /: *mut *mut __be32 cmd_csum_errs; / Command checksum errors,
    pub /: *mut *mut __be32 rx_pkts; / Rx NCSI packets,
    pub /: *mut *mut __be32 tx_pkts; / Tx NCSI packets,
    pub /: *mut *mut __be32 tx_aen_pkts; / Tx AEN packets,
    pub /: *mut *mut __be32 checksum; / Checksum,
}

// Get NCSI Pass-through Statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_rsp_gnpts_pkt {
    pub /: *mut *mut ncsi_rsp_pkt_hdr rsp; / Response header,
    pub /: *mut *mut __be32 tx_pkts; / Tx packets,
    pub /: *mut *mut __be32 tx_dropped; / Tx dropped packets,
    pub /: *mut *mut __be32 tx_channel_err; / Tx channel errors,
    pub /: *mut *mut __be32 tx_us_err; / Tx undersize errors,
    pub /: *mut *mut __be32 rx_pkts; / Rx packets,
    pub /: *mut *mut __be32 rx_dropped; / Rx dropped packets,
    pub /: *mut *mut __be32 rx_channel_err; / Rx channel errors,
    pub /: *mut *mut __be32 rx_us_err; / Rx undersize errors,
    pub /: *mut *mut __be32 rx_os_err; / Rx oversize errors,
    pub /: *mut *mut __be32 checksum; / Checksum,
}

// Get package status
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_rsp_gps_pkt {
    pub /: *mut *mut ncsi_rsp_pkt_hdr rsp; / Response header,
    pub /: *mut *mut __be32 status; / Hardware arbitration status,
    pub checksum: __be32,
}

// Get package UUID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_rsp_gpuuid_pkt {
    pub /: *mut *mut ncsi_rsp_pkt_hdr rsp; / Response header,
    pub /: *mut *mut unsigned char uuid[16]; / UUID,
    pub checksum: __be32,
}

// Get MC MAC Address
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_rsp_gmcma_pkt {
    pub rsp: ncsi_rsp_pkt_hdr,
    pub address_count: c_uchar,
    pub reserved: [c_uchar; 3],
    pub addresses: [c_uchar; ][ETH_ALEN],
}

// AEN: Link State Change
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_aen_lsc_pkt {
    pub /: *mut *mut ncsi_aen_pkt_hdr aen; / AEN header,
    pub /: *mut *mut __be32 status; / Link status,
    pub /: *mut *mut __be32 oem_status; / OEM link status,
    pub /: *mut *mut __be32 checksum; / Checksum,
    pub pad: [c_uchar; 14],
}

// AEN: Configuration Required
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_aen_cr_pkt {
    pub /: *mut *mut ncsi_aen_pkt_hdr aen; / AEN header,
    pub /: *mut *mut __be32 checksum; / Checksum,
    pub pad: [c_uchar; 22],
}

// AEN: Host Network Controller Driver Status Change
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_aen_hncdsc_pkt {
    pub /: *mut *mut ncsi_aen_pkt_hdr aen; / AEN header,
    pub /: *mut *mut __be32 status; / Status,
    pub /: *mut *mut __be32 checksum; / Checksum,
    pub pad: [c_uchar; 18],
}

// NCSI packet revision
pub const NCSI_PKT_REVISION: c_uint = 0x01;
// NCSI packet commands
pub const NCSI_PKT_CMD_CIS: c_uint = 0x00 /* Clear Initial State              */;
pub const NCSI_PKT_CMD_SP: c_uint = 0x01 /* Select Package                   */;
pub const NCSI_PKT_CMD_DP: c_uint = 0x02 /* Deselect Package                 */;
pub const NCSI_PKT_CMD_EC: c_uint = 0x03 /* Enable Channel                   */;
pub const NCSI_PKT_CMD_DC: c_uint = 0x04 /* Disable Channel                  */;
pub const NCSI_PKT_CMD_RC: c_uint = 0x05 /* Reset Channel                    */;
pub const NCSI_PKT_CMD_ECNT: c_uint = 0x06 /* Enable Channel Network Tx        */;
pub const NCSI_PKT_CMD_DCNT: c_uint = 0x07 /* Disable Channel Network Tx       */;
pub const NCSI_PKT_CMD_AE: c_uint = 0x08 /* AEN Enable                       */;
pub const NCSI_PKT_CMD_SL: c_uint = 0x09 /* Set Link                         */;
pub const NCSI_PKT_CMD_GLS: c_uint = 0x0a /* Get Link                         */;
pub const NCSI_PKT_CMD_SVF: c_uint = 0x0b /* Set VLAN Filter                  */;
pub const NCSI_PKT_CMD_EV: c_uint = 0x0c /* Enable VLAN                      */;
pub const NCSI_PKT_CMD_DV: c_uint = 0x0d /* Disable VLAN                     */;
pub const NCSI_PKT_CMD_SMA: c_uint = 0x0e /* Set MAC address                  */;
pub const NCSI_PKT_CMD_EBF: c_uint = 0x10 /* Enable Broadcast Filter          */;
pub const NCSI_PKT_CMD_DBF: c_uint = 0x11 /* Disable Broadcast Filter         */;
pub const NCSI_PKT_CMD_EGMF: c_uint = 0x12 /* Enable Global Multicast Filter   */;
pub const NCSI_PKT_CMD_DGMF: c_uint = 0x13 /* Disable Global Multicast Filter  */;
pub const NCSI_PKT_CMD_SNFC: c_uint = 0x14 /* Set NCSI Flow Control            */;
pub const NCSI_PKT_CMD_GVI: c_uint = 0x15 /* Get Version ID                   */;
pub const NCSI_PKT_CMD_GC: c_uint = 0x16 /* Get Capabilities                 */;
pub const NCSI_PKT_CMD_GP: c_uint = 0x17 /* Get Parameters                   */;
pub const NCSI_PKT_CMD_GCPS: c_uint = 0x18 /* Get Controller Packet Statistics */;
pub const NCSI_PKT_CMD_GNS: c_uint = 0x19 /* Get NCSI Statistics              */;
pub const NCSI_PKT_CMD_GNPTS: c_uint = 0x1a /* Get NCSI Pass-throu Statistics   */;
pub const NCSI_PKT_CMD_GPS: c_uint = 0x1b /* Get package status               */;
pub const NCSI_PKT_CMD_OEM: c_uint = 0x50 /* OEM                              */;
pub const NCSI_PKT_CMD_PLDM: c_uint = 0x51 /* PLDM request over NCSI over RBT  */;
pub const NCSI_PKT_CMD_GPUUID: c_uint = 0x52 /* Get package UUID                 */;
pub const NCSI_PKT_CMD_QPNPR: c_uint = 0x56 /* Query Pending NC PLDM request */;
pub const NCSI_PKT_CMD_SNPR: c_uint = 0x57 /* Send NC PLDM Reply  */;
pub const NCSI_PKT_CMD_GMCMA: c_uint = 0x58 /* Get MC MAC Address */;
// NCSI packet responses

// NCSI response code/reason
pub const NCSI_PKT_RSP_C_COMPLETED: c_uint = 0x0000 /* Command Completed        */;
pub const NCSI_PKT_RSP_C_FAILED: c_uint = 0x0001 /* Command Failed           */;
pub const NCSI_PKT_RSP_C_UNAVAILABLE: c_uint = 0x0002 /* Command Unavailable      */;
pub const NCSI_PKT_RSP_C_UNSUPPORTED: c_uint = 0x0003 /* Command Unsupported      */;
pub const NCSI_PKT_RSP_R_NO_ERROR: c_uint = 0x0000 /* No Error                 */;
pub const NCSI_PKT_RSP_R_INTERFACE: c_uint = 0x0001 /* Interface not ready      */;
pub const NCSI_PKT_RSP_R_PARAM: c_uint = 0x0002 /* Invalid Parameter        */;
pub const NCSI_PKT_RSP_R_CHANNEL: c_uint = 0x0003 /* Channel not Ready        */;
pub const NCSI_PKT_RSP_R_PACKAGE: c_uint = 0x0004 /* Package not Ready        */;
pub const NCSI_PKT_RSP_R_LENGTH: c_uint = 0x0005 /* Invalid payload length   */;
pub const NCSI_PKT_RSP_R_UNKNOWN: c_uint = 0x7fff /* Command type unsupported */;
// NCSI AEN packet type
pub const NCSI_PKT_AEN: c_uint = 0xFF /* AEN Packet               */;
pub const NCSI_PKT_AEN_LSC: c_uint = 0x00 /* Link status change       */;
pub const NCSI_PKT_AEN_CR: c_uint = 0x01 /* Configuration required   */;
pub const NCSI_PKT_AEN_HNCDSC: c_uint = 0x02 /* HNC driver status change */;
