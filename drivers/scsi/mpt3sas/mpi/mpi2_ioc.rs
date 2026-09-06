//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/mpt3sas/mpi/mpi2_ioc.h
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
// Copyright 2000-2020 Broadcom Inc. All rights reserved.
//
// Name:  mpi2_ioc.h
// Title:  MPI IOC, Port, Event, FW Download, and FW Upload messages
// Creation Date:  October 11, 2006
//
// mpi2_ioc.h Version:  02.00.37
//
// NOTE: Names (typedefs, defines, etc.) beginning with an MPI25 or Mpi25
// prefix are for use only on MPI v2.5 products, and must not be used
// with MPI v2.0 products. Unless otherwise noted, names beginning with
// MPI2 or Mpi2 are for use with both MPI v2.0 and MPI v2.5 products.
//
// Version History
// ---------------
//
// Date      Version   Description
// --------  --------  ------------------------------------------------------
// 04-30-07  02.00.00  Corresponds to Fusion-MPT MPI Specification Rev A.
// 06-04-07  02.00.01  In IOCFacts Reply structure, renamed MaxDevices to
// MaxTargets.
// Added TotalImageSize field to FWDownload Request.
// Added reserved words to FWUpload Request.
// 06-26-07  02.00.02  Added IR Configuration Change List Event.
// 08-31-07  02.00.03  Removed SystemReplyQueueDepth field from the IOCInit
// request and replaced it with
// ReplyDescriptorPostQueueDepth and ReplyFreeQueueDepth.
// Replaced the MinReplyQueueDepth field of the IOCFacts
// reply with MaxReplyDescriptorPostQueueDepth.
// Added MPI2_RDPQ_DEPTH_MIN define to specify the minimum
// depth for the Reply Descriptor Post Queue.
// Added SASAddress field to Initiator Device Table
// Overflow Event data.
// 10-31-07  02.00.04  Added ReasonCode MPI2_EVENT_SAS_INIT_RC_NOT_RESPONDING
// for SAS Initiator Device Status Change Event data.
// Modified Reason Code defines for SAS Topology Change
// List Event data, including adding a bit for PHY Vacant
// status, and adding a mask for the Reason Code.
// Added define for
// MPI2_EVENT_SAS_TOPO_ES_DELAY_NOT_RESPONDING.
// Added define for MPI2_EXT_IMAGE_TYPE_MEGARAID.
// 12-18-07  02.00.05  Added Boot Status defines for the IOCExceptions field of
// the IOCFacts Reply.
// Removed MPI2_IOCFACTS_CAPABILITY_EXTENDED_BUFFER define.
// Moved MPI2_VERSION_UNION to mpi2.h.
// Changed MPI2_EVENT_NOTIFICATION_REQUEST to use masks
// instead of enables, and added SASBroadcastPrimitiveMasks
// field.
// Added Log Entry Added Event and related structure.
// 02-29-08  02.00.06  Added define MPI2_IOCFACTS_CAPABILITY_INTEGRATED_RAID.
// Removed define MPI2_IOCFACTS_PROTOCOL_SMP_TARGET.
// Added MaxVolumes and MaxPersistentEntries fields to
// IOCFacts reply.
// Added ProtocalFlags and IOCCapabilities fields to
// MPI2_FW_IMAGE_HEADER.
// Removed MPI2_PORTENABLE_FLAGS_ENABLE_SINGLE_PORT.
// 03-03-08  02.00.07  Fixed MPI2_FW_IMAGE_HEADER by changing Reserved26 to
// a U16 (from a U32).
// Removed extra 's' from EventMasks name.
// 06-27-08  02.00.08  Fixed an offset in a comment.
// 10-02-08  02.00.09  Removed SystemReplyFrameSize from MPI2_IOC_INIT_REQUEST.
// Removed CurReplyFrameSize from MPI2_IOC_FACTS_REPLY and
// renamed MinReplyFrameSize to ReplyFrameSize.
// Added MPI2_IOCFACTS_EXCEPT_IR_FOREIGN_CONFIG_MAX.
// Added two new RAIDOperation values for Integrated RAID
// Operations Status Event data.
// Added four new IR Configuration Change List Event data
// ReasonCode values.
// Added two new ReasonCode defines for SAS Device Status
// Change Event data.
// Added three new DiscoveryStatus bits for the SAS
// Discovery event data.
// Added Multiplexing Status Change bit to the PhyStatus
// field of the SAS Topology Change List event data.
// Removed define for MPI2_INIT_IMAGE_BOOTFLAGS_XMEMCOPY.
// BootFlags are now product-specific.
// Added defines for the indivdual signature bytes
// for MPI2_INIT_IMAGE_FOOTER.
// 01-19-09  02.00.10  Added MPI2_IOCFACTS_CAPABILITY_EVENT_REPLAY define.
// Added MPI2_EVENT_SAS_DISC_DS_DOWNSTREAM_INITIATOR
// define.
// Added MPI2_EVENT_SAS_DEV_STAT_RC_SATA_INIT_FAILURE
// define.
// Removed MPI2_EVENT_SAS_DISC_DS_SATA_INIT_FAILURE define.
// 05-06-09  02.00.11  Added MPI2_IOCFACTS_CAPABILITY_RAID_ACCELERATOR define.
// Added MPI2_IOCFACTS_CAPABILITY_MSI_X_INDEX define.
// Added two new reason codes for SAS Device Status Change
// Event.
// Added new event: SAS PHY Counter.
// 07-30-09  02.00.12  Added GPIO Interrupt event define and structure.
// Added MPI2_IOCFACTS_CAPABILITY_EXTENDED_BUFFER define.
// Added new product id family for 2208.
// 10-28-09  02.00.13  Added HostMSIxVectors field to MPI2_IOC_INIT_REQUEST.
// Added MaxMSIxVectors field to MPI2_IOC_FACTS_REPLY.
// Added MinDevHandle field to MPI2_IOC_FACTS_REPLY.
// Added MPI2_IOCFACTS_CAPABILITY_HOST_BASED_DISCOVERY.
// Added MPI2_EVENT_HOST_BASED_DISCOVERY_PHY define.
// Added MPI2_EVENT_SAS_TOPO_ES_NO_EXPANDER define.
// Added Host Based Discovery Phy Event data.
// Added defines for ProductID Product field
// (MPI2_FW_HEADER_PID_).
// Modified values for SAS ProductID Family
// (MPI2_FW_HEADER_PID_FAMILY_).
// 02-10-10  02.00.14  Added SAS Quiesce Event structure and defines.
// Added PowerManagementControl Request structures and
// defines.
// 05-12-10  02.00.15  Marked Task Set Full Event as obsolete.
// Added MPI2_EVENT_SAS_TOPO_LR_UNSUPPORTED_PHY define.
// 11-10-10  02.00.16  Added MPI2_FW_DOWNLOAD_ITYPE_MIN_PRODUCT_SPECIFIC.
// 02-23-11  02.00.17  Added SAS NOTIFY Primitive event, and added
// SASNotifyPrimitiveMasks field to
// MPI2_EVENT_NOTIFICATION_REQUEST.
// Added Temperature Threshold Event.
// Added Host Message Event.
// Added Send Host Message request and reply.
// 05-25-11  02.00.18  For Extended Image Header, added
// MPI2_EXT_IMAGE_TYPE_MIN_PRODUCT_SPECIFIC and
// MPI2_EXT_IMAGE_TYPE_MAX_PRODUCT_SPECIFIC defines.
// Deprecated MPI2_EXT_IMAGE_TYPE_MAX define.
// 08-24-11  02.00.19  Added PhysicalPort field to
// MPI2_EVENT_DATA_SAS_DEVICE_STATUS_CHANGE structure.
// Marked MPI2_PM_CONTROL_FEATURE_PCIE_LINK as obsolete.
// 11-18-11  02.00.20  Incorporating additions for MPI v2.5.
// 03-29-12  02.00.21  Added a product specific range to event values.
// 07-26-12  02.00.22  Added MPI2_IOCFACTS_EXCEPT_PARTIAL_MEMORY_FAILURE.
// Added ElapsedSeconds field to
// MPI2_EVENT_DATA_IR_OPERATION_STATUS.
// 08-19-13  02.00.23  For IOCInit, added MPI2_IOCINIT_MSGFLAG_RDPQ_ARRAY_MODE
// and MPI2_IOC_INIT_RDPQ_ARRAY_ENTRY.
// Added MPI2_IOCFACTS_CAPABILITY_RDPQ_ARRAY_CAPABLE.
// Added MPI2_FW_DOWNLOAD_ITYPE_PUBLIC_KEY.
// Added Encrypted Hash Extended Image.
// 12-05-13  02.00.24  Added MPI25_HASH_IMAGE_TYPE_BIOS.
// 11-18-14  02.00.25  Updated copyright information.
// 03-16-15  02.00.26  Updated for MPI v2.6.
// Added MPI2_EVENT_ACTIVE_CABLE_EXCEPTION and
// MPI26_EVENT_DATA_ACTIVE_CABLE_EXCEPT.
// Added MPI26_FW_HEADER_PID_FAMILY_3324_SAS and
// MPI26_FW_HEADER_PID_FAMILY_3516_SAS.
// Added MPI26_CTRL_OP_SHUTDOWN.
// 08-25-15  02.00.27  Added IC ARCH Class based signature defines.
// Added MPI26_EVENT_PCIE_ENUM_ES_RESOURCES_EXHAUSTED event.
// Added ConigurationFlags field to IOCInit message to
// support NVMe SGL format control.
// Added PCIe SRIOV support.
// 02-17-16   02.00.28 Added SAS 4 22.5 gbs speed support.
// Added PCIe 4 16.0 GT/sec speec support.
// Removed AHCI support.
// Removed SOP support.
// 07-01-16   02.00.29 Added Archclass for 4008 product.
// Added IOCException MPI2_IOCFACTS_EXCEPT_PCIE_DISABLED
// 08-23-16   02.00.30 Added new defines for the ImageType field of FWDownload
// Request Message.
// Added new defines for the ImageType field of FWUpload
// Request Message.
// Added new values for the RegionType field in the Layout
// Data sections of the FLASH Layout Extended Image Data.
// Added new defines for the ReasonCode field of
// Active Cable Exception Event.
// Added MPI2_EVENT_ENCL_DEVICE_STATUS_CHANGE and
// MPI26_EVENT_DATA_ENCL_DEV_STATUS_CHANGE.
// 11-23-16   02.00.31 Added MPI2_EVENT_SAS_DEVICE_DISCOVERY_ERROR and
// MPI25_EVENT_DATA_SAS_DEVICE_DISCOVERY_ERROR.
// 02-02-17   02.00.32 Added MPI2_FW_DOWNLOAD_ITYPE_CBB_BACKUP.
// Added MPI25_EVENT_DATA_ACTIVE_CABLE_EXCEPT and related
// defines for the ReasonCode field.
// 06-13-17   02.00.33 Added MPI2_FW_DOWNLOAD_ITYPE_CPLD.
// 09-29-17   02.00.34 Added MPI26_EVENT_PCIDEV_STAT_RC_PCIE_HOT_RESET_FAILED
// to the ReasonCode field in PCIe Device Status Change
// Event Data.
// 07-22-18   02.00.35 Added FW_DOWNLOAD_ITYPE_CPLD and _PSOC.
// Moved FW image definitions ionto new mpi2_image,h
// 08-14-18   02.00.36 Fixed definition of MPI2_FW_DOWNLOAD_ITYPE_PSOC (0x16)
// 09-07-18   02.00.37 Added MPI26_EVENT_PCIE_TOPO_PI_16_LANES
// 10-02-19   02.00.38 Added MPI26_IOCINIT_CFGFLAGS_COREDUMP_ENABLE
// Added MPI26_IOCFACTS_CAPABILITY_COREDUMP_ENABLED
// Added MPI2_FW_DOWNLOAD_ITYPE_COREDUMP
// Added MPI2_FW_UPLOAD_ITYPE_COREDUMP
// 9-13-24    02.00.39 Added MPI26_MCTP_PASSTHROUGH messages
// --------------------------------------------------------------------------
//
// IOC Messages
//
// IOCInit message
//
// IOCInit Request message
// WhoInit values

// MsgFlags

// MsgVersion

// HeaderVersion

// ConfigurationFlags

// minimum depth for a Reply Descriptor Post Queue

// Reply Descriptor Post Queue Array Entry
// PTR_MPI2_IOC_INIT_RDPQ_ARRAY_ENTRY,
// IOCInit Reply message
//
// IOCFacts message
//
// IOCFacts Request message
// IOCFacts Reply message
// MsgVersion

// HeaderVersion

// IOCExceptions

// defines for WhoInit field are after the IOCInit Request
// ProductID field uses MPI2_FW_HEADER_PID_
// IOCCapabilities

// ProtocolFlags

//
// PortFacts message
//
// PortFacts Request message
// PortFacts Reply message
// PortType values

//
// PortEnable message
//
// PortEnable Request message
// PortEnable Reply message
//
// EventNotification message
//
// EventNotification Request message

// PTR_MPI2_EVENT_NOTIFICATION_REQUEST,
// pMpi2EventNotificationRequest_t;
// EventNotification Reply message
// pMpi2EventNotificationReply_t;
// AckRequired

// Event

// Log Entry Added Event data
// the following structure matches MPI2_LOG_0_ENTRY in mpi2_cnfg.h

// PTR_MPI2_EVENT_DATA_LOG_ENTRY_ADDED,
// pMpi2EventDataLogEntryAdded_t;
// GPIO Interrupt Event data
// PTR_MPI2_EVENT_DATA_GPIO_INTERRUPT,
// pMpi2EventDataGpioInterrupt_t;
// Temperature Threshold Event data
// PTR_MPI2_EVENT_DATA_TEMPERATURE,
// Temperature Threshold Event data Status bits

// Host Message Event data
// Power Performance Change Event data
// PTR_MPI2_EVENT_DATA_POWER_PERF_CHANGE,
// pMpi2EventDataPowerPerfChange_t;
// defines for CurrentPowerMode and PreviousPowerMode fields

// Active Cable Exception Event data
// PTR_MPI25_EVENT_DATA_ACTIVE_CABLE_EXCEPT,
// pMpi25EventDataActiveCableExcept_t,
// PTR_MPI26_EVENT_DATA_ACTIVE_CABLE_EXCEPT,
// pMpi26EventDataActiveCableExcept_t;
// MPI2.5 defines for the ReasonCode field

// defines for ReasonCode field

// Hard Reset Received Event data
// PTR_MPI2_EVENT_DATA_HARD_RESET_RECEIVED,
// pMpi2EventDataHardResetReceived_t;
// Task Set Full Event data
// this event is obsolete
// SAS Device Status Change Event data
// PTR_MPI2_EVENT_DATA_SAS_DEVICE_STATUS_CHANGE,
// pMpi2EventDataSasDeviceStatusChange_t;
// SAS Device Status Change Event data ReasonCode values

// Integrated RAID Operation Status Event data
// PTR_MPI2_EVENT_DATA_IR_OPERATION_STATUS,
// pMpi2EventDataIrOperationStatus_t;
// Integrated RAID Operation Status Event data RAIDOperation values

// Integrated RAID Volume Event data
// Integrated RAID Volume Event data ReasonCode values

// Integrated RAID Physical Disk Event data
// PTR_MPI2_EVENT_DATA_IR_PHYSICAL_DISK,
// pMpi2EventDataIrPhysicalDisk_t;
// Integrated RAID Physical Disk Event data ReasonCode values

// Integrated RAID Configuration Change List Event data
//
// Host code (drivers, BIOS, utilities, etc.) should check NumElements at
// runtime before using ConfigElement[].
//
// IR Configuration Change List Event data ElementFlags values

// IR Configuration Change List Event data ReasonCode values

// PTR_MPI2_EVENT_DATA_IR_CONFIG_CHANGE_LIST,
// pMpi2EventDataIrConfigChangeList_t;
// IR Configuration Change List Event data Flags values

// SAS Discovery Event data
// PTR_MPI2_EVENT_DATA_SAS_DISCOVERY,
// SAS Discovery Event data Flags values

// SAS Discovery Event data ReasonCode values

// SAS Discovery Event data DiscoveryStatus values

// SAS Broadcast Primitive Event data
// PTR_MPI2_EVENT_DATA_SAS_BROADCAST_PRIMITIVE,
// pMpi2EventDataSasBroadcastPrimitive_t;
// defines for the Primitive field

// SAS Notify Primitive Event data
// PTR_MPI2_EVENT_DATA_SAS_NOTIFY_PRIMITIVE,
// pMpi2EventDataSasNotifyPrimitive_t;
// defines for the Primitive field

// SAS Initiator Device Status Change Event data
// PTR_MPI2_EVENT_DATA_SAS_INIT_DEV_STATUS_CHANGE,
// pMpi2EventDataSasInitDevStatusChange_t;
// SAS Initiator Device Status Change event ReasonCode values

// SAS Initiator Device Table Overflow Event data
// PTR_MPI2_EVENT_DATA_SAS_INIT_TABLE_OVERFLOW,
// pMpi2EventDataSasInitTableOverflow_t;
// SAS Topology Change List Event data
//
// Host code (drivers, BIOS, utilities, etc.) should check NumEntries at
// runtime before using PHY[].
//
// PTR_MPI2_EVENT_DATA_SAS_TOPOLOGY_CHANGE_LIST,
// pMpi2EventDataSasTopologyChangeList_t;
// values for the ExpStatus field

// defines for the LinkRate field

// values for the PhyStatus field

// values for the PhyStatus ReasonCode sub-field

// SAS Enclosure Device Status Change Event data
// PTR_MPI2_EVENT_DATA_SAS_ENCL_DEV_STATUS_CHANGE,
// pMpi2EventDataSasEnclDevStatusChange_t,
// PTR_MPI26_EVENT_DATA_ENCL_DEV_STATUS_CHANGE,
// pMpi26EventDataEnclDevStatusChange_t;
// SAS Enclosure Device Status Change event ReasonCode values

// Enclosure Device Status Change event ReasonCode values

// PTR_MPI25_EVENT_DATA_SAS_DEVICE_DISCOVERY_ERROR,
// pMpi25EventDataSasDeviceDiscoveryError_t;
// SAS Device Discovery Error Event data ReasonCode values

// SAS PHY Counter Event data
// PTR_MPI2_EVENT_DATA_SAS_PHY_COUNTER,
// pMpi2EventDataSasPhyCounter_t;
// use MPI2_SASPHY3_EVENT_CODE_ values from mpi2_cnfg.h
// for the PhyEventCode field
// use MPI2_SASPHY3_COUNTER_TYPE_ values from mpi2_cnfg.h
// for the CounterType field
// use MPI2_SASPHY3_TIME_UNITS_ values from mpi2_cnfg.h
// for the TimeUnits field
// use MPI2_SASPHY3_TFLAGS_ values from mpi2_cnfg.h
// for the ThresholdFlags field
// SAS Quiesce Event data
// PTR_MPI2_EVENT_DATA_SAS_QUIESCE,
// SAS Quiesce Event data ReasonCode values

// Host Based Discovery Phy Event data
// values for the Flags field

// use MPI2_SAS_NEG_LINK_RATE_ defines from mpi2_cnfg.h
// for the NegotiatedLinkRate field
// pMpi2EventDataMpi2EventDataHbdPhy_t;
// values for the DescriptorType field

// PCIe Device Status Change Event data (MPI v2.6 and later)
// PTR_MPI26_EVENT_DATA_PCIE_DEVICE_STATUS_CHANGE,
// pMpi26EventDataPCIeDeviceStatusChange_t;
// PCIe Device Status Change Event data ReasonCode values

// PCIe Enumeration Event data (MPI v2.6 and later)
// PTR_MPI26_EVENT_DATA_PCIE_ENUMERATION,
// pMpi26EventDataPCIeEnumeration_t;
// PCIe Enumeration Event data Flags values

// PCIe Enumeration Event data ReasonCode values

// PCIe Enumeration Event data EnumerationStatus values

// PCIe Topology Change List Event data (MPI v2.6 and later)
//
// Host code (drivers, BIOS, utilities, etc.) should check NumEntries at
// runtime before using PortEntry[].
//
// PTR_MPI26_EVENT_PCIE_TOPO_PORT_ENTRY,
// pMpi26EventPCIeTopoPortEntry_t;
// PCIe Topology Change List Event data PortStatus values

// PCIe Topology Change List Event data defines for CurrentPortInfo and
// PreviousPortInfo
//

// PTR_MPI26_EVENT_DATA_PCIE_TOPOLOGY_CHANGE_LIST,
// pMpi26EventDataPCIeTopologyChangeList_t;
// PCIe Topology Change List Event data SwitchStatus values

// PCIe Link Counter Event data (MPI v2.6 and later)
// PTR_MPI26_EVENT_DATA_PCIE_LINK_COUNTER,
// use MPI26_PCIELINK3_EVTCODE_ values from mpi2_cnfg.h for the LinkEventCode
// field
//
// use MPI26_PCIELINK3_COUNTER_TYPE_ values from mpi2_cnfg.h for the CounterType
// field
//
// use MPI26_PCIELINK3_TIME_UNITS_ values from mpi2_cnfg.h for the TimeUnits
// field
//
// use MPI26_PCIELINK3_TFLAGS_ values from mpi2_cnfg.h for the ThresholdFlags
// field
//
// EventAck message
//
// EventAck Request message
// EventAck Reply message
//
// SendHostMessage message
//
// SendHostMessage Request message
// PTR_MPI2_SEND_HOST_MESSAGE_REQUEST,
// pMpi2SendHostMessageRequest_t;
// SendHostMessage Reply message
//
// FWDownload message
//
// MPI v2.0 FWDownload Request message

// MPI v2.6 and newer

// MPI v2.0 FWDownload TransactionContext Element
// MPI v2.5 FWDownload Request message
// FWDownload Reply message
//
// FWUpload message
//
// MPI v2.0 FWUpload Request message

// MPI v2.0 FWUpload TransactionContext Element
// MPI v2.5 FWUpload Request message
// FWUpload Reply message
//
// PowerManagementControl message
//
// PowerManagementControl Request message
// defines for the Feature field

// parameter usage for the MPI2_PM_CONTROL_FEATURE_DA_PHY_POWER_COND Feature
// Parameter1 contains a PHY number
// Parameter2 indicates power condition action using these defines

// Parameter3 and Parameter4 are reserved
// parameter usage for the MPI2_PM_CONTROL_FEATURE_PORT_WIDTH_MODULATION
// Feature
// Parameter1 contains SAS port width modulation group number
// Parameter2 indicates IOC action using these defines

// Parameter3 indicates desired modulation level using these defines

// Parameter4 is reserved
// this next set (_PCIE_LINK) is obsolete
// parameter usage for the MPI2_PM_CONTROL_FEATURE_PCIE_LINK Feature
// Parameter1 indicates desired PCIe link speed using these defines

// Parameter2 indicates desired PCIe link width using these defines

// Parameter3 and Parameter4 are reserved
// parameter usage for the MPI2_PM_CONTROL_FEATURE_IOC_SPEED Feature
// Parameter1 indicates desired IOC hardware clock speed using these defines

// Parameter2, Parameter3, and Parameter4 are reserved
// parameter usage for the MPI2_PM_CONTROL_FEATURE_GLOBAL_PWR_MGMT_MODE Feature
// Parameter1 indicates host action regarding global power management mode

// Parameter2 indicates the requested global power management mode

// Parameter3 and Parameter4 are reserved
// PowerManagementControl Reply message
//
// IO Unit Control messages (MPI v2.6 and later only.)
//
// IO Unit Control Request Message
// PTR_MPI26_IOUNIT_CONTROL_REQUEST,
// pMpi26IoUnitControlRequest_t;
// values for the Operation field

// values for the PrimFlags field

// values for the LookupMethod field

// IO Unit Control Reply Message
// PTR_MPI26_IOUNIT_CONTROL_REPLY,
// pMpi26IoUnitControlReply_t;
//
// MCTP Passthrough messages (MPI v2.6 and later only.)
//
// MCTP Passthrough Request Message
// PTR_MPI26_MCTP_PASSTHROUGH_REQUEST,
// pMpi26MctpPassthroughRequest_t;
// values for the MsgContext field

// values for the Flags field

// MCTP Passthrough Reply Message
// PTR_MPI26_MCTP_PASSTHROUGH_REPLY,
// pMpi26MctpPassthroughReply_t;
