//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/rdma/rdma_netlink.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

// The minimum version that the iwpm kernel supports
pub const IWPM_UABI_VERSION_MIN: c_int = 3;
// The latest version that the iwpm kernel supports
pub const IWPM_UABI_VERSION: c_int = 4;
// iwarp port mapper message flags
// Do not map the port for this IWPM request
// iwarp port mapper op-codes
// The following maintains bisectability of rdma-core
pub const IWPM_NLA_MAPINFO_SEND_MAX: c_int = 3;
pub const IWPM_NLA_REMOVE_MAPPING_MAX: c_int = 3;
// For RDMA_NLDEV_ATTR_DEV_NODE_TYPE
// IB values map to NodeInfo:NodeType.
//
// Local service operations:
// RESOLVE - The client requests the local service to resolve a path.
// SET_TIMEOUT - The local service requests the client to set the timeout.
// IP_RESOLVE - The client requests the local service to resolve an IP to GID.
//
// Local service netlink message flags
pub const RDMA_NL_LS_F_ERR: c_uint = 0x0100	/* Failed response */;
//
// Local service resolve operation family header.
// The layout for the resolve operation:
// nlmsg header
// family header
// attributes
//
// Local service path use:
// Specify how the path(s) will be used.
// ALL - For connected CM operation (6 pathrecords)
// UNIDIRECTIONAL - For unidirectional UD (1 pathrecord)
// GMP - For miscellaneous GMP like operation (at least 1 reversible
// pathrecord)
//
pub const LS_DEVICE_NAME_MAX: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ls_resolve_header {
    pub device_name: [__u8; LS_DEVICE_NAME_MAX],
    pub port_num: __u8,
    pub path_use: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ls_ip_resolve_header {
    pub ifindex: __u32,
}

// Local service attribute type

//
// Local service attributes:
// Attr Name       Size                       Byte order
// -----------------------------------------------------
// PATH_RECORD     struct ib_path_rec_data
// TIMEOUT         u32                        cpu
// SERVICE_ID      u64                        cpu
// DGID            u8[16]                     BE
// SGID            u8[16]                     BE
// TCLASS          u8
// PKEY            u16                        cpu
// QOS_CLASS       u16                        cpu
// IPV4            u32                        BE
// IPV6            u8[16]                     BE
//
// Local service DGID/SGID attribute: big endian
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_nla_ls_gid {
    pub gid: [__u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_nldev_command {
    RDMA_NLDEV_CMD_UNSPEC,

    RDMA_NLDEV_CMD_GET, /* can dump */
    RDMA_NLDEV_CMD_SET,

    RDMA_NLDEV_CMD_NEWLINK,

    RDMA_NLDEV_CMD_DELLINK,

    RDMA_NLDEV_CMD_PORT_GET, /* can dump */

    RDMA_NLDEV_CMD_SYS_GET,
    RDMA_NLDEV_CMD_SYS_SET,

// 8 is free to use

    RDMA_NLDEV_CMD_RES_GET = 9, /* can dump */

    RDMA_NLDEV_CMD_RES_QP_GET, /* can dump */

    RDMA_NLDEV_CMD_RES_CM_ID_GET, /* can dump */

    RDMA_NLDEV_CMD_RES_CQ_GET, /* can dump */

    RDMA_NLDEV_CMD_RES_MR_GET, /* can dump */

    RDMA_NLDEV_CMD_RES_PD_GET, /* can dump */

    RDMA_NLDEV_CMD_GET_CHARDEV,

    RDMA_NLDEV_CMD_STAT_SET,

    RDMA_NLDEV_CMD_STAT_GET, /* can dump */

    RDMA_NLDEV_CMD_STAT_DEL,

    RDMA_NLDEV_CMD_RES_QP_GET_RAW,

    RDMA_NLDEV_CMD_RES_CQ_GET_RAW,

    RDMA_NLDEV_CMD_RES_MR_GET_RAW,

    RDMA_NLDEV_CMD_RES_CTX_GET, /* can dump */

    RDMA_NLDEV_CMD_RES_SRQ_GET, /* can dump */

    RDMA_NLDEV_CMD_STAT_GET_STATUS,

    RDMA_NLDEV_CMD_RES_SRQ_GET_RAW,

    RDMA_NLDEV_CMD_NEWDEV,

    RDMA_NLDEV_CMD_DELDEV,

    RDMA_NLDEV_CMD_MONITOR,

    RDMA_NLDEV_CMD_FRMR_POOLS_GET, /* can dump */

    RDMA_NLDEV_CMD_FRMR_POOLS_SET,

    RDMA_NLDEV_NUM_OPS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_nldev_print_type {
    RDMA_NLDEV_PRINT_TYPE_UNSPEC,
    RDMA_NLDEV_PRINT_TYPE_HEX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_nldev_attr {
// don't change the order or add anything between, this is ABI!
    RDMA_NLDEV_ATTR_UNSPEC,

// Pad attribute for 64b alignment
    RDMA_NLDEV_ATTR_PAD = RDMA_NLDEV_ATTR_UNSPEC,

// Identifier for ib_device
    RDMA_NLDEV_ATTR_DEV_INDEX,		/* u32 */

    RDMA_NLDEV_ATTR_DEV_NAME,		/* string */
//
// Device index together with port index are identifiers
// for port/link properties.
//
// For RDMA_NLDEV_CMD_GET commamnd, port index will return number
// of available ports in ib_device, while for port specific operations,
// it will be real port index as it appears in sysfs. Port index follows
// sysfs notation and starts from 1 for the first port.
//
    RDMA_NLDEV_ATTR_PORT_INDEX,		/* u32 */

//
// Device and port capabilities
//
// When used for port info, first 32-bits are CapabilityMask followed by
// 16-bit CapabilityMask2.
//
    RDMA_NLDEV_ATTR_CAP_FLAGS,		/* u64 */

//
// FW version
//
    RDMA_NLDEV_ATTR_FW_VERSION,		/* string */

//
// Node GUID (in host byte order) associated with the RDMA device.
//
    RDMA_NLDEV_ATTR_NODE_GUID,			/* u64 */

//
// System image GUID (in host byte order) associated with
// this RDMA device and other devices which are part of a
// single system.
//
    RDMA_NLDEV_ATTR_SYS_IMAGE_GUID,		/* u64 */

//
// Subnet prefix (in host byte order)
//
    RDMA_NLDEV_ATTR_SUBNET_PREFIX,		/* u64 */

//
// Local Identifier (LID),
// According to IB specification, It is 16-bit address assigned
// by the Subnet Manager. Extended to be 32-bit for OmniPath users.
//
    RDMA_NLDEV_ATTR_LID,			/* u32 */
    RDMA_NLDEV_ATTR_SM_LID,			/* u32 */

//
// LID mask control (LMC)
//
    RDMA_NLDEV_ATTR_LMC,			/* u8 */

    RDMA_NLDEV_ATTR_PORT_STATE,		/* u8 */
    RDMA_NLDEV_ATTR_PORT_PHYS_STATE,	/* u8 */

    RDMA_NLDEV_ATTR_DEV_NODE_TYPE,		/* u8 */

    RDMA_NLDEV_ATTR_RES_SUMMARY,		/* nested table */
    RDMA_NLDEV_ATTR_RES_SUMMARY_ENTRY,	/* nested table */
    RDMA_NLDEV_ATTR_RES_SUMMARY_ENTRY_NAME,	/* string */
    RDMA_NLDEV_ATTR_RES_SUMMARY_ENTRY_CURR,	/* u64 */

    RDMA_NLDEV_ATTR_RES_QP,			/* nested table */
    RDMA_NLDEV_ATTR_RES_QP_ENTRY,		/* nested table */
//
// Local QPN
//
    RDMA_NLDEV_ATTR_RES_LQPN,		/* u32 */
//
// Remote QPN,
// Applicable for RC and UC only IBTA 11.2.5.3 QUERY QUEUE PAIR
//
    RDMA_NLDEV_ATTR_RES_RQPN,		/* u32 */
//
// Receive Queue PSN,
// Applicable for RC and UC only 11.2.5.3 QUERY QUEUE PAIR
//
    RDMA_NLDEV_ATTR_RES_RQ_PSN,		/* u32 */
//
// Send Queue PSN
//
    RDMA_NLDEV_ATTR_RES_SQ_PSN,		/* u32 */
    RDMA_NLDEV_ATTR_RES_PATH_MIG_STATE,	/* u8 */
//
// QP types as visible to RDMA/core, the reserved QPT
// are not exported through this interface.
//
    RDMA_NLDEV_ATTR_RES_TYPE,		/* u8 */
    RDMA_NLDEV_ATTR_RES_STATE,		/* u8 */
//
// Process ID which created object,
// in case of kernel origin, PID won't exist.
//
    RDMA_NLDEV_ATTR_RES_PID,		/* u32 */
//
// The name of process created following resource.
// It will exist only for kernel objects.
// For user created objects, the user is supposed
// to read /proc/PID/comm file.
//
    RDMA_NLDEV_ATTR_RES_KERN_NAME,		/* string */

    RDMA_NLDEV_ATTR_RES_CM_ID,		/* nested table */
    RDMA_NLDEV_ATTR_RES_CM_ID_ENTRY,	/* nested table */
//
// rdma_cm_id port space.
//
    RDMA_NLDEV_ATTR_RES_PS,			/* u32 */
//
// Source and destination socket addresses
//
    RDMA_NLDEV_ATTR_RES_SRC_ADDR,		/* __kernel_sockaddr_storage */
    RDMA_NLDEV_ATTR_RES_DST_ADDR,		/* __kernel_sockaddr_storage */

    RDMA_NLDEV_ATTR_RES_CQ,			/* nested table */
    RDMA_NLDEV_ATTR_RES_CQ_ENTRY,		/* nested table */
    RDMA_NLDEV_ATTR_RES_CQE,		/* u32 */
    RDMA_NLDEV_ATTR_RES_USECNT,		/* u64 */
    RDMA_NLDEV_ATTR_RES_POLL_CTX,		/* u8 */

    RDMA_NLDEV_ATTR_RES_MR,			/* nested table */
    RDMA_NLDEV_ATTR_RES_MR_ENTRY,		/* nested table */
    RDMA_NLDEV_ATTR_RES_RKEY,		/* u32 */
    RDMA_NLDEV_ATTR_RES_LKEY,		/* u32 */
    RDMA_NLDEV_ATTR_RES_IOVA,		/* u64 */
    RDMA_NLDEV_ATTR_RES_MRLEN,		/* u64 */

    RDMA_NLDEV_ATTR_RES_PD,			/* nested table */
    RDMA_NLDEV_ATTR_RES_PD_ENTRY,		/* nested table */
    RDMA_NLDEV_ATTR_RES_LOCAL_DMA_LKEY,	/* u32 */
    RDMA_NLDEV_ATTR_RES_UNSAFE_GLOBAL_RKEY,	/* u32 */
//
// Provides logical name and index of netdevice which is
// connected to physical port. This information is relevant
// for RoCE and iWARP.
//
// The netdevices which are associated with containers are
// supposed to be exported together with GID table once it
// will be exposed through the netlink. Because the
// associated netdevices are properties of GIDs.
//
    RDMA_NLDEV_ATTR_NDEV_INDEX,		/* u32 */
    RDMA_NLDEV_ATTR_NDEV_NAME,		/* string */
//
// driver-specific attributes.
//
    RDMA_NLDEV_ATTR_DRIVER,			/* nested table */
    RDMA_NLDEV_ATTR_DRIVER_ENTRY,		/* nested table */
    RDMA_NLDEV_ATTR_DRIVER_STRING,		/* string */
//
// u8 values from enum rdma_nldev_print_type
//
    RDMA_NLDEV_ATTR_DRIVER_PRINT_TYPE,	/* u8 */
    RDMA_NLDEV_ATTR_DRIVER_S32,		/* s32 */
    RDMA_NLDEV_ATTR_DRIVER_U32,		/* u32 */
    RDMA_NLDEV_ATTR_DRIVER_S64,		/* s64 */
    RDMA_NLDEV_ATTR_DRIVER_U64,		/* u64 */

//
// Indexes to get/set secific entry,
// for QP use RDMA_NLDEV_ATTR_RES_LQPN
//
    RDMA_NLDEV_ATTR_RES_PDN,               /* u32 */
    RDMA_NLDEV_ATTR_RES_CQN,               /* u32 */
    RDMA_NLDEV_ATTR_RES_MRN,               /* u32 */
    RDMA_NLDEV_ATTR_RES_CM_IDN,            /* u32 */
    RDMA_NLDEV_ATTR_RES_CTXN,	       /* u32 */
//
// Identifies the rdma driver. eg: "rxe" or "siw"
//
    RDMA_NLDEV_ATTR_LINK_TYPE,		/* string */

//
// net namespace mode for rdma subsystem:
// either shared or exclusive among multiple net namespaces.
//
    RDMA_NLDEV_SYS_ATTR_NETNS_MODE,		/* u8 */
//
// Device protocol, e.g. ib, iw, usnic, roce and opa
//
    RDMA_NLDEV_ATTR_DEV_PROTOCOL,		/* string */

//
// File descriptor handle of the net namespace object. May be combined
// with RDMA_NLDEV_ATTR_DEV_NAME (a literal device name) to also rename
// the device in the destination namespace; the move fails with -EEXIST
// if that name is already taken there.
//
    RDMA_NLDEV_NET_NS_FD,			/* u32 */
//
// Information about a chardev.
// CHARDEV_TYPE is the name of the chardev ABI (ie uverbs, umad, etc)
// CHARDEV_ABI signals the ABI revision (historical)
// CHARDEV_NAME is the kernel name for the /dev/ file (no directory)
// CHARDEV is the 64 bit dev_t for the inode
//
    RDMA_NLDEV_ATTR_CHARDEV_TYPE,		/* string */
    RDMA_NLDEV_ATTR_CHARDEV_NAME,		/* string */
    RDMA_NLDEV_ATTR_CHARDEV_ABI,		/* u64 */
    RDMA_NLDEV_ATTR_CHARDEV,		/* u64 */
    RDMA_NLDEV_ATTR_UVERBS_DRIVER_ID,       /* u64 */
//
// Counter-specific attributes.
//
    RDMA_NLDEV_ATTR_STAT_MODE,		/* u32 */
    RDMA_NLDEV_ATTR_STAT_RES,		/* u32 */
    RDMA_NLDEV_ATTR_STAT_AUTO_MODE_MASK,	/* u32 */
    RDMA_NLDEV_ATTR_STAT_COUNTER,		/* nested table */
    RDMA_NLDEV_ATTR_STAT_COUNTER_ENTRY,	/* nested table */
    RDMA_NLDEV_ATTR_STAT_COUNTER_ID,	/* u32 */
    RDMA_NLDEV_ATTR_STAT_HWCOUNTERS,	/* nested table */
    RDMA_NLDEV_ATTR_STAT_HWCOUNTER_ENTRY,	/* nested table */
    RDMA_NLDEV_ATTR_STAT_HWCOUNTER_ENTRY_NAME,	/* string */
    RDMA_NLDEV_ATTR_STAT_HWCOUNTER_ENTRY_VALUE,	/* u64 */

//
// CQ adaptive moderatio (DIM)
//
    RDMA_NLDEV_ATTR_DEV_DIM,                /* u8 */

    RDMA_NLDEV_ATTR_RES_RAW,	/* binary */

    RDMA_NLDEV_ATTR_RES_CTX,		/* nested table */
    RDMA_NLDEV_ATTR_RES_CTX_ENTRY,		/* nested table */

    RDMA_NLDEV_ATTR_RES_SRQ,		/* nested table */
    RDMA_NLDEV_ATTR_RES_SRQ_ENTRY,		/* nested table */
    RDMA_NLDEV_ATTR_RES_SRQN,		/* u32 */

    RDMA_NLDEV_ATTR_MIN_RANGE,		/* u32 */
    RDMA_NLDEV_ATTR_MAX_RANGE,		/* u32 */

    RDMA_NLDEV_SYS_ATTR_COPY_ON_FORK,	/* u8 */

    RDMA_NLDEV_ATTR_STAT_HWCOUNTER_INDEX,	/* u32 */
    RDMA_NLDEV_ATTR_STAT_HWCOUNTER_DYNAMIC, /* u8 */

    RDMA_NLDEV_SYS_ATTR_PRIVILEGED_QKEY_MODE, /* u8 */

    RDMA_NLDEV_ATTR_DRIVER_DETAILS,		/* u8 */
//
// QP subtype string, used for driver QPs
//
    RDMA_NLDEV_ATTR_RES_SUBTYPE,		/* string */

    RDMA_NLDEV_ATTR_DEV_TYPE,		/* u8 */

    RDMA_NLDEV_ATTR_PARENT_NAME,		/* string */

    RDMA_NLDEV_ATTR_NAME_ASSIGN_TYPE,	/* u8 */

    RDMA_NLDEV_ATTR_EVENT_TYPE,		/* u8 */

    RDMA_NLDEV_SYS_ATTR_MONITOR_MODE,	/* u8 */

    RDMA_NLDEV_ATTR_STAT_OPCOUNTER_ENABLED,	/* u8 */

//
// FRMR Pools attributes
//
    RDMA_NLDEV_ATTR_FRMR_POOLS,		/* nested table */
    RDMA_NLDEV_ATTR_FRMR_POOL_ENTRY,	/* nested table */
    RDMA_NLDEV_ATTR_FRMR_POOL_KEY,		/* nested table */
    RDMA_NLDEV_ATTR_FRMR_POOL_KEY_ATS,	/* u8 */
    RDMA_NLDEV_ATTR_FRMR_POOL_KEY_ACCESS_FLAGS,	/* u32 */
    RDMA_NLDEV_ATTR_FRMR_POOL_KEY_VENDOR_KEY,	/* u64 */
    RDMA_NLDEV_ATTR_FRMR_POOL_KEY_NUM_DMA_BLOCKS,	/* u64 */
    RDMA_NLDEV_ATTR_FRMR_POOL_QUEUE_HANDLES,	/* u32 */
    RDMA_NLDEV_ATTR_FRMR_POOL_MAX_IN_USE,	/* u64 */
    RDMA_NLDEV_ATTR_FRMR_POOL_IN_USE,	/* u64 */
    RDMA_NLDEV_ATTR_FRMR_POOLS_AGING_PERIOD,	/* u32 */
    RDMA_NLDEV_ATTR_FRMR_POOL_PINNED_HANDLES,	/* u32 */
    RDMA_NLDEV_ATTR_FRMR_POOL_KEY_KERNEL_VENDOR_KEY,	/* u64 */

//
// Resource summary entry maximum value.
//
    RDMA_NLDEV_ATTR_RES_SUMMARY_ENTRY_MAX,		/* u64 */

//
// Always the end
//
    RDMA_NLDEV_ATTR_MAX
}

//
// Supported counter bind modes. All modes are mutual-exclusive.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_nl_counter_mode {
    RDMA_COUNTER_MODE_NONE,

//
// A qp is bound with a counter automatically during initialization
// based on the auto mode (e.g., qp type, ...)
//
    RDMA_COUNTER_MODE_AUTO,

//
// Which qp are bound with which counter is explicitly specified
// by the user
//
    RDMA_COUNTER_MODE_MANUAL,

//
// Always the end
//
    RDMA_COUNTER_MODE_MAX,
}

//
// Supported criteria in counter auto mode.
// Currently only "qp type" is supported
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_nl_counter_mask {
    RDMA_COUNTER_MASK_QP_TYPE = 1,
    RDMA_COUNTER_MASK_PID = 1 << 1,
}

// Supported rdma device types.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_nl_dev_type {
    RDMA_DEVICE_TYPE_SMI = 1,
}

// RDMA device name assignment types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_nl_name_assign_type {
    RDMA_NAME_ASSIGN_TYPE_UNKNOWN = 0,
    RDMA_NAME_ASSIGN_TYPE_USER = 1, /* Provided by user-space */
}

//
// Supported rdma monitoring event types.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_nl_notify_event_type {
    RDMA_REGISTER_EVENT,
    RDMA_UNREGISTER_EVENT,
    RDMA_NETDEV_ATTACH_EVENT,
    RDMA_NETDEV_DETACH_EVENT,
    RDMA_RENAME_EVENT,
    RDMA_NETDEV_RENAME_EVENT,
}
