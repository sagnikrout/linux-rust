//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/message/fusion/lsi/mpi_ioc.h
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
// Copyright (c) 2000-2008 LSI Corporation.
//
// Name:  mpi_ioc.h
// Title:  MPI IOC, Port, Event, FW Download, and FW Upload messages
// Creation Date:  August 11, 2000
//
// mpi_ioc.h Version:  01.05.16
//
// Version History
// ---------------
//
// Date      Version   Description
// --------  --------  ------------------------------------------------------
// 05-08-00  00.10.01  Original release for 0.10 spec dated 4/26/2000.
// 05-24-00  00.10.02  Added _MSG_IOC_INIT_REPLY structure.
// 06-06-00  01.00.01  Added CurReplyFrameSize field to _MSG_IOC_FACTS_REPLY.
// 06-12-00  01.00.02  Added _MSG_PORT_ENABLE_REPLY structure.
// Added _MSG_EVENT_ACK_REPLY structure.
// Added _MSG_FW_DOWNLOAD_REPLY structure.
// Added _MSG_TOOLBOX_REPLY structure.
// 06-30-00  01.00.03  Added MaxLanBuckets to _PORT_FACT_REPLY structure.
// 07-27-00  01.00.04  Added _EVENT_DATA structure definitions for _SCSI,
// _LINK_STATUS, _LOOP_STATE and _LOGOUT.
// 08-11-00  01.00.05  Switched positions of MsgLength and Function fields in
// _MSG_EVENT_ACK_REPLY structure to match specification.
// 11-02-00  01.01.01  Original release for post 1.0 work.
// Added a value for Manufacturer to WhoInit.
// 12-04-00  01.01.02  Modified IOCFacts reply, added FWUpload messages, and
// removed toolbox message.
// 01-09-01  01.01.03  Added event enabled and disabled defines.
// Added structures for FwHeader and DataHeader.
// Added ImageType to FwUpload reply.
// 02-20-01  01.01.04  Started using MPI_POINTER.
// 02-27-01  01.01.05  Added event for RAID status change and its event data.
// Added IocNumber field to MSG_IOC_FACTS_REPLY.
// 03-27-01  01.01.06  Added defines for ProductId field of MPI_FW_HEADER.
// Added structure offset comments.
// 04-09-01  01.01.07  Added structure EVENT_DATA_EVENT_CHANGE.
// 08-08-01  01.02.01  Original release for v1.2 work.
// New format for FWVersion and ProductId in
// MSG_IOC_FACTS_REPLY and MPI_FW_HEADER.
// 08-31-01  01.02.02  Addded event MPI_EVENT_SCSI_DEVICE_STATUS_CHANGE and
// related structure and defines.
// Added event MPI_EVENT_ON_BUS_TIMER_EXPIRED.
// Added MPI_IOCINIT_FLAGS_DISCARD_FW_IMAGE.
// Replaced a reserved field in MSG_IOC_FACTS_REPLY with
// IOCExceptions and changed DataImageSize to reserved.
// Added MPI_FW_DOWNLOAD_ITYPE_NVSTORE_DATA and
// MPI_FW_UPLOAD_ITYPE_NVDATA.
// 09-28-01  01.02.03  Modified Event Data for Integrated RAID.
// 11-01-01  01.02.04  Added defines for MPI_EXT_IMAGE_HEADER ImageType field.
// 03-14-02  01.02.05  Added HeaderVersion field to MSG_IOC_FACTS_REPLY.
// 05-31-02  01.02.06  Added define for
// MPI_IOCFACTS_EXCEPT_RAID_CONFIG_INVALID.
// Added AliasIndex to EVENT_DATA_LOGOUT structure.
// 04-01-03  01.02.07  Added defines for MPI_FW_HEADER_SIGNATURE_.
// 06-26-03  01.02.08  Added new values to the product family defines.
// 04-29-04  01.02.09  Added IOCCapabilities field to MSG_IOC_FACTS_REPLY and
// added related defines.
// 05-11-04  01.03.01  Original release for MPI v1.3.
// 08-19-04  01.05.01  Added four new fields to MSG_IOC_INIT.
// Added three new fields to MSG_IOC_FACTS_REPLY.
// Defined four new bits for the IOCCapabilities field of
// the IOCFacts reply.
// Added two new PortTypes for the PortFacts reply.
// Added six new events along with their EventData
// structures.
// Added a new MsgFlag to the FwDownload request to
// indicate last segment.
// Defined a new image type of boot loader.
// Added FW family codes for SAS product families.
// 10-05-04  01.05.02  Added ReplyFifoHostSignalingAddr field to
// MSG_IOC_FACTS_REPLY.
// 12-07-04  01.05.03  Added more defines for SAS Discovery Error event.
// 12-09-04  01.05.04  Added Unsupported device to SAS Device event.
// 01-15-05  01.05.05  Added event data for SAS SES Event.
// 02-09-05  01.05.06  Added MPI_FW_UPLOAD_ITYPE_FW_BACKUP define.
// 02-22-05  01.05.07  Added Host Page Buffer Persistent flag to IOC Facts
// Reply and IOC Init Request.
// 03-11-05  01.05.08  Added family code for 1068E family.
// Removed IOCFacts Reply EEDP Capability bit.
// 06-24-05  01.05.09  Added 5 new IOCFacts Reply IOCCapabilities bits.
// Added Max SATA Targets to SAS Discovery Error event.
// 08-30-05  01.05.10  Added 4 new events and their event data structures.
// Added new ReasonCode value for SAS Device Status Change
// event.
// Added new family code for FC949E.
// 03-27-06  01.05.11  Added MPI_IOCFACTS_CAPABILITY_TLR.
// Added additional Reason Codes and more event data fields
// to EVENT_DATA_SAS_DEVICE_STATUS_CHANGE.
// Added EVENT_DATA_SAS_BROADCAST_PRIMITIVE structure and
// new event.
// Added MPI_EVENT_SAS_SMP_ERROR and event data structure.
// Added MPI_EVENT_SAS_INIT_DEVICE_STATUS_CHANGE and event
// data structure.
// Added MPI_EVENT_SAS_INIT_TABLE_OVERFLOW and event
// data structure.
// Added MPI_EXT_IMAGE_TYPE_INITIALIZATION.
// 10-11-06  01.05.12  Added MPI_IOCFACTS_EXCEPT_METADATA_UNSUPPORTED.
// Added MaxInitiators field to PortFacts reply.
// Added SAS Device Status Change ReasonCode for
// asynchronous notificaiton.
// Added MPI_EVENT_SAS_EXPANDER_STATUS_CHANGE and event
// data structure.
// Added new ImageType values for FWDownload and FWUpload
// requests.
// 02-28-07  01.05.13  Added MPI_EVENT_PRIMITIVE_ASYNCHRONOUS_EVENT for SAS
// Broadcast Event Data (replacing _RESERVED2).
// For Discovery Error Event Data DiscoveryStatus field,
// replaced _MULTPL_PATHS with _UNSUPPORTED_DEVICE and
// added _MULTI_PORT_DOMAIN.
// 05-24-07  01.05.14  Added Common Boot Block type to FWDownload Request.
// Added Common Boot Block type to FWUpload Request.
// 08-07-07  01.05.15  Added MPI_EVENT_SAS_INIT_RC_REMOVED define.
// Added MPI_EVENT_IR2_RC_DUAL_PORT_ADDED and
// MPI_EVENT_IR2_RC_DUAL_PORT_REMOVED for IR2 event data.
// Added SASAddress field to SAS Initiator Device Table
// Overflow event data structure.
// 03-28-08  01.05.16  Added two new ReasonCode values to SAS Device Status
// Change Event data to indicate completion of internally
// generated task management.
// Added MPI_EVENT_DSCVRY_ERR_DS_SATA_INIT_FAILURE define.
// Added MPI_EVENT_SAS_INIT_RC_INACCESSIBLE define.
// --------------------------------------------------------------------------
//
// I O C    M e s s a g e s
//
// IOCInit message
//
// WhoInit values

// Flags values

// MsgVersion

// HeaderVersion

//
// IOC Facts message
//
// IOC Facts Reply

//
// P o r t    M e s s a g e s
//
// Port Facts message and Reply
//
// PortTypes values

// ProtocolFlags values

//
// Port Enable Message
//
// E v e n t    M e s s a g e s
//
// Event Notification messages
//
// Event Notification Reply
// Event Acknowledge
// Switch

// Event

// AckRequired field values

// EventChange Event data
// LogEntryAdded Event data
// this structure matches MPI_LOG_0_ENTRY in mpi_cnfg.h

// SCSI Event data for Port, Bus and Device forms
// SCSI Device Status Change Event data
// MPI SCSI Device Status Change Event data ReasonCode values

// SAS Device Status Change Event data
// MPI SAS Device Status Change Event data ReasonCode values

// SCSI Event data for Queue Full event
// MPI Integrated RAID Event data
// MPI Integrated RAID Event data ReasonCode values

// MPI Integrated RAID Resync Update Event data
// MPI IR2 Event data
// MPI_LD_STATE or MPI_PD_STATE
// MPI IR2 Event data ReasonCode values

// defines for logical disk states

// defines for physical disk states

// MPI Link Status Change Event data

// MPI Loop State Change Event data

// MPI LOGOUT Event data

// SAS SES Event data
// SAS Broadcast Primitive Event data

// SAS Phy Link Status Event data
// defines for the LinkRates field of the SAS PHY Link Status event

// SAS Discovery Event data

// SAS Discovery Error Event data

// SAS SMP Error Event data
// defines for the Status field of the SAS SMP Error event

// SAS Initiator Device Status Change Event data
// defines for the ReasonCode field of the SAS Initiator Device Status Change event

// SAS Initiator Device Table Overflow Event data
// SAS Expander Status Change Event data
// values for ReasonCode field of SAS Expander Status Change Event data

// values for DiscoveryStatus field of SAS Expander Status Change Event data

// values for Flags field of SAS Expander Status Change Event data

//
// F i r m w a r e    L o a d    M e s s a g e s
//
// Firmware Download message and associated structures
//

// Firmware Download reply
//
// Firmware Upload message and associated structures
//

// Firmware Upload reply

// defines for using the ProductId field

// SCSI

// Fibre Channel

// SAS

// defines for the ImageType field

