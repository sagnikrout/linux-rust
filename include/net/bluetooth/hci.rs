//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/bluetooth/hci.h
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
pub const HCI_MAX_ACL_SIZE: c_int = 1024;
pub const HCI_MAX_SCO_SIZE: c_int = 255;
pub const HCI_MAX_ISO_SIZE: c_int = 251;
pub const HCI_MAX_ISO_BIS: c_int = 31;
pub const HCI_MAX_EVENT_SIZE: c_int = 260;

pub const HCI_LINK_KEY_SIZE: c_int = 16;
pub const HCI_MAX_CPB_DATA_SIZE: c_int = 252;
// HCI dev events
pub const HCI_DEV_REG: c_int = 1;
pub const HCI_DEV_UNREG: c_int = 2;
pub const HCI_DEV_UP: c_int = 3;
pub const HCI_DEV_DOWN: c_int = 4;
pub const HCI_DEV_SUSPEND: c_int = 5;
pub const HCI_DEV_RESUME: c_int = 6;
pub const HCI_DEV_OPEN: c_int = 7;
pub const HCI_DEV_CLOSE: c_int = 8;
pub const HCI_DEV_SETUP: c_int = 9;
// HCI notify events
pub const HCI_NOTIFY_CONN_ADD: c_int = 1;
pub const HCI_NOTIFY_CONN_DEL: c_int = 2;
pub const HCI_NOTIFY_VOICE_SETTING: c_int = 3;
pub const HCI_NOTIFY_ENABLE_SCO_CVSD: c_int = 4;
pub const HCI_NOTIFY_ENABLE_SCO_TRANSP: c_int = 5;
pub const HCI_NOTIFY_DISABLE_SCO: c_int = 6;
// HCI bus types
pub const HCI_VIRTUAL: c_int = 0;
pub const HCI_USB: c_int = 1;
pub const HCI_PCCARD: c_int = 2;
pub const HCI_UART: c_int = 3;
pub const HCI_RS232: c_int = 4;
pub const HCI_PCI: c_int = 5;
pub const HCI_SDIO: c_int = 6;
pub const HCI_SPI: c_int = 7;
pub const HCI_I2C: c_int = 8;
pub const HCI_SMD: c_int = 9;
pub const HCI_VIRTIO: c_int = 10;
pub const HCI_IPC: c_int = 11;
// HCI device quirks
// When this quirk is set, the HCI Reset command is send when
// closing the transport instead of when opening it.
//
// This quirk must be set before hci_register_dev is called.
//
// When this quirk is set, the device is turned into a raw-only
// device and it will stay in unconfigured state.
//
// This quirk must be set before hci_register_dev is called.
//
// When this quirk is set, the buffer sizes reported by
// HCI Read Buffer Size command are corrected if invalid.
//
// This quirk must be set before hci_register_dev is called.
//
// When this quirk is set, then a controller that does not
// indicate support for Inquiry Result with RSSI is assumed to
// support it anyway. Some early Bluetooth 1.2 controllers had
// wrongly configured local features that will require forcing
// them to enable this mode. Getting RSSI information with the
// inquiry responses is preferred since it allows for a better
// user experience.
//
// This quirk must be set before hci_register_dev is called.
//
// When this quirk is set, then the HCI Read Local Supported
// Commands command is not supported. In general Bluetooth 1.2
// and later controllers should support this command. However
// some controllers indicate Bluetooth 1.2 support, but do
// not support this command.
//
// This quirk must be set before hci_register_dev is called.
//
// When this quirk is set, then no stored link key handling
// is performed. This is mainly due to the fact that the
// HCI Delete Stored Link Key command is advertised, but
// not supported.
//
// This quirk must be set before hci_register_dev is called.
//
// When this quirk is set, an external configuration step
// is required and will be indicated with the controller
// configuration.
//
// This quirk can be set before hci_register_dev is called or
// during the hdev->setup vendor callback.
//
// When this quirk is set, the public Bluetooth address
// initially reported by HCI Read BD Address command
// is considered invalid. Controller configuration is
// required before this device can be used.
//
// This quirk can be set before hci_register_dev is called or
// during the hdev->setup vendor callback.
//
// When this quirk is set, the public Bluetooth address
// initially reported by HCI Read BD Address command
// is considered invalid. The public BD Address can be
// specified in the fwnode property 'local-bd-address'.
// If this property does not exist or is invalid controller
// configuration is required before this device can be used.
//
// This quirk can be set before hci_register_dev is called or
// during the hdev->setup vendor callback.
//
// When this quirk is set, the Bluetooth Device Address provided by
// the 'local-bd-address' fwnode property is incorrectly specified in
// big-endian order.
//
// This quirk can be set before hci_register_dev is called or
// during the hdev->setup vendor callback.
//
// When this quirk is set, the duplicate filtering during
// scanning is based on Bluetooth devices addresses. To allow
// RSSI based updates, restart scanning if needed.
//
// This quirk can be set before hci_register_dev is called or
// during the hdev->setup vendor callback.
//
// When this quirk is set, LE scan and BR/EDR inquiry is done
// simultaneously, otherwise it's interleaved.
//
// This quirk can be set before hci_register_dev is called or
// during the hdev->setup vendor callback.
//
// When this quirk is set, the enabling of diagnostic mode is
// not persistent over HCI Reset. Every time the controller
// is brought up it needs to be reprogrammed.
//
// This quirk can be set before hci_register_dev is called or
// during the hdev->setup vendor callback.
//
// When this quirk is set, setup() would be run after every
// open() and not just after the first open().
//
// This quirk can be set before hci_register_dev is called or
// during the hdev->setup vendor callback.
//
// When this quirk is set, wide band speech is supported by
// the driver since no reliable mechanism exist to report
// this from the hardware, a driver flag is use to convey
// this support
//
// This quirk must be set before hci_register_dev is called.
//
// When this quirk is set consider Sync Flow Control as supported by
// the driver.
//
// This quirk must be set before hci_register_dev is called.
//
// When this quirk is set, the LE states reported through the
// HCI_LE_READ_SUPPORTED_STATES are invalid/broken.
//
// This mechanism is necessary as many controllers have been seen has
// having trouble initiating a connectable advertisement despite the
// state combination being reported as supported.
//
// This quirk can be set before hci_register_dev is called or
// during the hdev->setup vendor callback.
//
// When this quirk is set, then erroneous data reporting
// is ignored. This is mainly due to the fact that the HCI
// Read Default Erroneous Data Reporting command is advertised,
// but not supported; these controllers often reply with unknown
// command and tend to lock up randomly. Needing a hard reset.
//
// This quirk can be set before hci_register_dev is called or
// during the hdev->setup vendor callback.
//
// When this quirk is set, then the hci_suspend_notifier is not
// registered. This is intended for devices which drop completely
// from the bus on system-suspend and which will show up as a new
// HCI after resume.
//
// When this quirk is set, LE tx power is not queried on startup
// and the min/max tx power values default to HCI_TX_POWER_INVALID.
//
// This quirk can be set before hci_register_dev is called or
// during the hdev->setup vendor callback.
//
// When this quirk is set, HCI_OP_SET_EVENT_FLT requests with
// HCI_FLT_CLEAR_ALL are ignored and event filtering is
// completely avoided. A subset of the CSR controller
// clones struggle with this and instantly lock up.
//
// Note that devices using this must (separately) disable
// runtime suspend, because event filtering takes place there.
//
// When this quirk is set, disables the use of
// HCI_OP_ENHANCED_SETUP_SYNC_CONN command to setup SCO connections.
//
// This quirk can be set before hci_register_dev is called or
// during the hdev->setup vendor callback.
//
// When this quirk is set, the HCI_OP_LE_SET_EXT_SCAN_ENABLE command is
// disabled. This is required for some Broadcom controllers which
// erroneously claim to support extended scanning.
//
// This quirk can be set before hci_register_dev is called or
// during the hdev->setup vendor callback.
//
// When this quirk is set, the HCI_OP_GET_MWS_TRANSPORT_CONFIG command is
// disabled. This is required for some Broadcom controllers which
// erroneously claim to support MWS Transport Layer Configuration.
//
// This quirk can be set before hci_register_dev is called or
// during the hdev->setup vendor callback.
//
// When this quirk is set, max_page for local extended features
// is set to 1, even if controller reports higher number. Some
// controllers (e.g. RTL8723CS) report more pages, but they
// don't actually support features declared there.
//
// When this quirk is set, the HCI_OP_LE_SET_RPA_TIMEOUT command is
// skipped during initialization. This is required for the Actions
// Semiconductor ATS2851 based controllers, which erroneously claims
// to support it.
//
// When this quirk is set, the HCI_OP_LE_EXT_CREATE_CONN command is
// disabled. This is required for the Actions Semiconductor ATS2851
// based controllers, which erroneously claims to support it.
//
// When this quirk is set, the command WRITE_AUTH_PAYLOAD_TIMEOUT is
// skipped. This is required for the Actions Semiconductor ATS2851
// based controllers, due to a race condition in pairing process.
//
// When this quirk is set, MSFT extension monitor tracking by
// address filter is supported. Since tracking quantity of each
// pattern is limited, this feature supports tracking multiple
// devices concurrently if controller supports multiple
// address filters.
//
// This quirk must be set before hci_register_dev is called.
//
// When this quirk is set, LE Coded PHY shall not be used. This is
// required for some Intel controllers which erroneously claim to
// support it but it causes problems with extended scanning.
//
// This quirk can be set before hci_register_dev is called or
// during the hdev->setup vendor callback.
//
// When this quirk is set, the HCI_OP_READ_ENC_KEY_SIZE command is
// skipped during an HCI_EV_ENCRYPT_CHANGE event. This is required
// for Actions Semiconductor ATS2851 based controllers, which erroneously
// claim to support it.
//
// When this quirk is set, the reserved bits of Primary/Secondary_PHY
// inside the LE Extended Advertising Report events are discarded.
// This is required for some Apple/Broadcom controllers which
// abuse these reserved bits for unrelated flags.
//
// This quirk can be set before hci_register_dev is called or
// during the hdev->setup vendor callback.
//
// When this quirk is set, the HCI_OP_READ_VOICE_SETTING command is
// skipped. This is required for a subset of the CSR controller clones
// which erroneously claim to support it.
//
// This quirk must be set before hci_register_dev is called.
//
// When this quirk is set, the HCI_OP_READ_PAGE_SCAN_TYPE command is
// skipped. This is required for a subset of the CSR controller clones
// which erroneously claim to support it.
//
// This quirk must be set before hci_register_dev is called.
//
// HCI device flags
// HCI socket flags
//
// BR/EDR and/or LE controller flags: the flags defined here should represent
// states from the controller.
//
// HCI timeouts

// HCI data types
pub const HCI_COMMAND_PKT: c_uint = 0x01;
pub const HCI_ACLDATA_PKT: c_uint = 0x02;
pub const HCI_SCODATA_PKT: c_uint = 0x03;
pub const HCI_EVENT_PKT: c_uint = 0x04;
pub const HCI_ISODATA_PKT: c_uint = 0x05;
pub const HCI_DIAG_PKT: c_uint = 0xf0;
pub const HCI_DRV_PKT: c_uint = 0xf1;
pub const HCI_VENDOR_PKT: c_uint = 0xff;
// HCI packet types
pub const HCI_DM1: c_uint = 0x0008;
pub const HCI_DM3: c_uint = 0x0400;
pub const HCI_DM5: c_uint = 0x4000;
pub const HCI_DH1: c_uint = 0x0010;
pub const HCI_DH3: c_uint = 0x0800;
pub const HCI_DH5: c_uint = 0x8000;
// HCI packet types inverted masks
pub const HCI_2DH1: c_uint = 0x0002;
pub const HCI_3DH1: c_uint = 0x0004;
pub const HCI_2DH3: c_uint = 0x0100;
pub const HCI_3DH3: c_uint = 0x0200;
pub const HCI_2DH5: c_uint = 0x1000;
pub const HCI_3DH5: c_uint = 0x2000;
pub const HCI_HV1: c_uint = 0x0020;
pub const HCI_HV2: c_uint = 0x0040;
pub const HCI_HV3: c_uint = 0x0080;

// eSCO packet types
pub const ESCO_HV1: c_uint = 0x0001;
pub const ESCO_HV2: c_uint = 0x0002;
pub const ESCO_HV3: c_uint = 0x0004;
pub const ESCO_EV3: c_uint = 0x0008;
pub const ESCO_EV4: c_uint = 0x0010;
pub const ESCO_EV5: c_uint = 0x0020;
pub const ESCO_2EV3: c_uint = 0x0040;
pub const ESCO_3EV3: c_uint = 0x0080;
pub const ESCO_2EV5: c_uint = 0x0100;
pub const ESCO_3EV5: c_uint = 0x0200;

// ACL flags
pub const ACL_START_NO_FLUSH: c_uint = 0x00;
pub const ACL_CONT: c_uint = 0x01;
pub const ACL_START: c_uint = 0x02;
pub const ACL_COMPLETE: c_uint = 0x03;
pub const ACL_ACTIVE_BCAST: c_uint = 0x04;
pub const ACL_PICO_BCAST: c_uint = 0x08;
// ISO PB flags
pub const ISO_START: c_uint = 0x00;
pub const ISO_CONT: c_uint = 0x01;
pub const ISO_SINGLE: c_uint = 0x02;
pub const ISO_END: c_uint = 0x03;
// ISO TS flags
pub const ISO_TS: c_uint = 0x01;
// Baseband links
pub const SCO_LINK: c_uint = 0x00;
pub const ACL_LINK: c_uint = 0x01;
pub const ESCO_LINK: c_uint = 0x02;
// Low Energy links do not have defined link type. Use invented one
pub const LE_LINK: c_uint = 0x80;
pub const CIS_LINK: c_uint = 0x82;
pub const BIS_LINK: c_uint = 0x83;
pub const PA_LINK: c_uint = 0x84;
pub const INVALID_LINK: c_uint = 0xff;
// LMP features
pub const LMP_3SLOT: c_uint = 0x01;
pub const LMP_5SLOT: c_uint = 0x02;
pub const LMP_ENCRYPT: c_uint = 0x04;
pub const LMP_SOFFSET: c_uint = 0x08;
pub const LMP_TACCURACY: c_uint = 0x10;
pub const LMP_RSWITCH: c_uint = 0x20;
pub const LMP_HOLD: c_uint = 0x40;
pub const LMP_SNIFF: c_uint = 0x80;
pub const LMP_PARK: c_uint = 0x01;
pub const LMP_RSSI: c_uint = 0x02;
pub const LMP_QUALITY: c_uint = 0x04;
pub const LMP_SCO: c_uint = 0x08;
pub const LMP_HV2: c_uint = 0x10;
pub const LMP_HV3: c_uint = 0x20;
pub const LMP_ULAW: c_uint = 0x40;
pub const LMP_ALAW: c_uint = 0x80;
pub const LMP_CVSD: c_uint = 0x01;
pub const LMP_PSCHEME: c_uint = 0x02;
pub const LMP_PCONTROL: c_uint = 0x04;
pub const LMP_TRANSPARENT: c_uint = 0x08;
pub const LMP_EDR_2M: c_uint = 0x02;
pub const LMP_EDR_3M: c_uint = 0x04;
pub const LMP_RSSI_INQ: c_uint = 0x40;
pub const LMP_ESCO: c_uint = 0x80;
pub const LMP_EV4: c_uint = 0x01;
pub const LMP_EV5: c_uint = 0x02;
pub const LMP_NO_BREDR: c_uint = 0x20;
pub const LMP_LE: c_uint = 0x40;
pub const LMP_EDR_3SLOT: c_uint = 0x80;
pub const LMP_EDR_5SLOT: c_uint = 0x01;
pub const LMP_SNIFF_SUBR: c_uint = 0x02;
pub const LMP_PAUSE_ENC: c_uint = 0x04;
pub const LMP_EDR_ESCO_2M: c_uint = 0x20;
pub const LMP_EDR_ESCO_3M: c_uint = 0x40;
pub const LMP_EDR_3S_ESCO: c_uint = 0x80;
pub const LMP_EXT_INQ: c_uint = 0x01;
pub const LMP_SIMUL_LE_BR: c_uint = 0x02;
pub const LMP_SIMPLE_PAIR: c_uint = 0x08;
pub const LMP_ERR_DATA_REPORTING: c_uint = 0x20;
pub const LMP_NO_FLUSH: c_uint = 0x40;
pub const LMP_LSTO: c_uint = 0x01;
pub const LMP_INQ_TX_PWR: c_uint = 0x02;
pub const LMP_EXTFEATURES: c_uint = 0x80;
// Extended LMP features
pub const LMP_CPB_CENTRAL: c_uint = 0x01;
pub const LMP_CPB_PERIPHERAL: c_uint = 0x02;
pub const LMP_SYNC_TRAIN: c_uint = 0x04;
pub const LMP_SYNC_SCAN: c_uint = 0x08;
pub const LMP_SC: c_uint = 0x01;
pub const LMP_PING: c_uint = 0x02;
// Host features
pub const LMP_HOST_SSP: c_uint = 0x01;
pub const LMP_HOST_LE: c_uint = 0x02;
pub const LMP_HOST_LE_BREDR: c_uint = 0x04;
pub const LMP_HOST_SC: c_uint = 0x08;
// LE features
pub const HCI_LE_ENCRYPTION: c_uint = 0x01;
pub const HCI_LE_CONN_PARAM_REQ_PROC: c_uint = 0x02;
pub const HCI_LE_PERIPHERAL_FEATURES: c_uint = 0x08;
pub const HCI_LE_PING: c_uint = 0x10;
pub const HCI_LE_DATA_LEN_EXT: c_uint = 0x20;
pub const HCI_LE_LL_PRIVACY: c_uint = 0x40;
pub const HCI_LE_EXT_SCAN_POLICY: c_uint = 0x80;
pub const HCI_LE_PHY_2M: c_uint = 0x01;
pub const HCI_LE_PHY_CODED: c_uint = 0x08;
pub const HCI_LE_EXT_ADV: c_uint = 0x10;
pub const HCI_LE_PERIODIC_ADV: c_uint = 0x20;
pub const HCI_LE_CHAN_SEL_ALG2: c_uint = 0x40;
pub const HCI_LE_PAST_SENDER: c_uint = 0x01;
pub const HCI_LE_PAST_RECEIVER: c_uint = 0x02;
pub const HCI_LE_CIS_CENTRAL: c_uint = 0x10;
pub const HCI_LE_CIS_PERIPHERAL: c_uint = 0x20;
pub const HCI_LE_ISO_BROADCASTER: c_uint = 0x40;
pub const HCI_LE_ISO_SYNC_RECEIVER: c_uint = 0x80;
pub const HCI_LE_LL_EXT_FEATURE: c_uint = 0x80;
pub const HCI_LE_CS: c_uint = 0x40;
pub const HCI_LE_CS_HOST: c_uint = 0x80;
pub const HCI_LE_SCI: c_uint = 0x01	/* byte 9 - Shorter Connection Intervals */;
pub const HCI_LE_SCI_HOST: c_uint = 0x02	/* byte 9 - Shorter Connection Intervals (Host) */;
// Connection modes
pub const HCI_CM_ACTIVE: c_uint = 0x0000;
pub const HCI_CM_HOLD: c_uint = 0x0001;
pub const HCI_CM_SNIFF: c_uint = 0x0002;
pub const HCI_CM_PARK: c_uint = 0x0003;
// Link policies
pub const HCI_LP_RSWITCH: c_uint = 0x0001;
pub const HCI_LP_HOLD: c_uint = 0x0002;
pub const HCI_LP_SNIFF: c_uint = 0x0004;
pub const HCI_LP_PARK: c_uint = 0x0008;
// Link modes
pub const HCI_LM_ACCEPT: c_uint = 0x8000;
pub const HCI_LM_MASTER: c_uint = 0x0001;
pub const HCI_LM_AUTH: c_uint = 0x0002;
pub const HCI_LM_ENCRYPT: c_uint = 0x0004;
pub const HCI_LM_TRUSTED: c_uint = 0x0008;
pub const HCI_LM_RELIABLE: c_uint = 0x0010;
pub const HCI_LM_SECURE: c_uint = 0x0020;
pub const HCI_LM_FIPS: c_uint = 0x0040;
// Authentication types
pub const HCI_AT_NO_BONDING: c_uint = 0x00;
pub const HCI_AT_NO_BONDING_MITM: c_uint = 0x01;
pub const HCI_AT_DEDICATED_BONDING: c_uint = 0x02;
pub const HCI_AT_DEDICATED_BONDING_MITM: c_uint = 0x03;
pub const HCI_AT_GENERAL_BONDING: c_uint = 0x04;
pub const HCI_AT_GENERAL_BONDING_MITM: c_uint = 0x05;
// I/O capabilities
pub const HCI_IO_DISPLAY_ONLY: c_uint = 0x00;
pub const HCI_IO_DISPLAY_YESNO: c_uint = 0x01;
pub const HCI_IO_KEYBOARD_ONLY: c_uint = 0x02;
pub const HCI_IO_NO_INPUT_OUTPUT: c_uint = 0x03;
// Link Key types
pub const HCI_LK_COMBINATION: c_uint = 0x00;
pub const HCI_LK_LOCAL_UNIT: c_uint = 0x01;
pub const HCI_LK_REMOTE_UNIT: c_uint = 0x02;
pub const HCI_LK_DEBUG_COMBINATION: c_uint = 0x03;
pub const HCI_LK_UNAUTH_COMBINATION_P192: c_uint = 0x04;
pub const HCI_LK_AUTH_COMBINATION_P192: c_uint = 0x05;
pub const HCI_LK_CHANGED_COMBINATION: c_uint = 0x06;
pub const HCI_LK_UNAUTH_COMBINATION_P256: c_uint = 0x07;
pub const HCI_LK_AUTH_COMBINATION_P256: c_uint = 0x08;
// ---- HCI Error Codes ----
pub const HCI_ERROR_UNKNOWN_CONN_ID: c_uint = 0x02;
pub const HCI_ERROR_AUTH_FAILURE: c_uint = 0x05;
pub const HCI_ERROR_PIN_OR_KEY_MISSING: c_uint = 0x06;
pub const HCI_ERROR_MEMORY_EXCEEDED: c_uint = 0x07;
pub const HCI_ERROR_CONNECTION_TIMEOUT: c_uint = 0x08;
pub const HCI_ERROR_COMMAND_DISALLOWED: c_uint = 0x0c;
pub const HCI_ERROR_REJ_LIMITED_RESOURCES: c_uint = 0x0d;
pub const HCI_ERROR_REJ_BAD_ADDR: c_uint = 0x0f;
pub const HCI_ERROR_INVALID_PARAMETERS: c_uint = 0x12;
pub const HCI_ERROR_REMOTE_USER_TERM: c_uint = 0x13;
pub const HCI_ERROR_REMOTE_LOW_RESOURCES: c_uint = 0x14;
pub const HCI_ERROR_REMOTE_POWER_OFF: c_uint = 0x15;
pub const HCI_ERROR_LOCAL_HOST_TERM: c_uint = 0x16;
pub const HCI_ERROR_PAIRING_NOT_ALLOWED: c_uint = 0x18;
pub const HCI_ERROR_UNSUPPORTED_REMOTE_FEATURE: c_uint = 0x1a;
pub const HCI_ERROR_INVALID_LL_PARAMS: c_uint = 0x1e;
pub const HCI_ERROR_UNSPECIFIED: c_uint = 0x1f;
pub const HCI_ERROR_ADVERTISING_TIMEOUT: c_uint = 0x3c;
pub const HCI_ERROR_CANCELLED_BY_HOST: c_uint = 0x44;
// Flow control modes
pub const HCI_FLOW_CTL_MODE_PACKET_BASED: c_uint = 0x00;
pub const HCI_FLOW_CTL_MODE_BLOCK_BASED: c_uint = 0x01;
// The core spec defines 127 as the "not available" value
pub const HCI_TX_POWER_INVALID: c_int = 127;
pub const HCI_RSSI_INVALID: c_int = 127;
pub const HCI_SYNC_HANDLE_INVALID: c_uint = 0xffff;
pub const HCI_SID_INVALID: c_uint = 0xff;
pub const HCI_ROLE_MASTER: c_uint = 0x00;
pub const HCI_ROLE_SLAVE: c_uint = 0x01;
// Extended Inquiry Response field types
pub const EIR_FLAGS: c_uint = 0x01 /* flags */;
pub const EIR_UUID16_SOME: c_uint = 0x02 /* 16-bit UUID, more available */;
pub const EIR_UUID16_ALL: c_uint = 0x03 /* 16-bit UUID, all listed */;
pub const EIR_UUID32_SOME: c_uint = 0x04 /* 32-bit UUID, more available */;
pub const EIR_UUID32_ALL: c_uint = 0x05 /* 32-bit UUID, all listed */;
pub const EIR_UUID128_SOME: c_uint = 0x06 /* 128-bit UUID, more available */;
pub const EIR_UUID128_ALL: c_uint = 0x07 /* 128-bit UUID, all listed */;
pub const EIR_NAME_SHORT: c_uint = 0x08 /* shortened local name */;
pub const EIR_NAME_COMPLETE: c_uint = 0x09 /* complete local name */;
pub const EIR_TX_POWER: c_uint = 0x0A /* transmit power level */;
pub const EIR_CLASS_OF_DEV: c_uint = 0x0D /* Class of Device */;
pub const EIR_SSP_HASH_C192: c_uint = 0x0E /* Simple Pairing Hash C-192 */;
pub const EIR_SSP_RAND_R192: c_uint = 0x0F /* Simple Pairing Randomizer R-192 */;
pub const EIR_DEVICE_ID: c_uint = 0x10 /* device ID */;
pub const EIR_APPEARANCE: c_uint = 0x19 /* Device appearance */;
pub const EIR_SERVICE_DATA: c_uint = 0x16 /* Service Data */;
pub const EIR_LE_BDADDR: c_uint = 0x1B /* LE Bluetooth device address */;
pub const EIR_LE_ROLE: c_uint = 0x1C /* LE role */;
pub const EIR_SSP_HASH_C256: c_uint = 0x1D /* Simple Pairing Hash C-256 */;
pub const EIR_SSP_RAND_R256: c_uint = 0x1E /* Simple Pairing Rand R-256 */;
pub const EIR_LE_SC_CONFIRM: c_uint = 0x22 /* LE SC Confirmation Value */;
pub const EIR_LE_SC_RANDOM: c_uint = 0x23 /* LE SC Random Value */;
// Low Energy Advertising Flags
pub const LE_AD_LIMITED: c_uint = 0x01 /* Limited Discoverable */;
pub const LE_AD_GENERAL: c_uint = 0x02 /* General Discoverable */;
pub const LE_AD_NO_BREDR: c_uint = 0x04 /* BR/EDR not supported */;
pub const LE_AD_SIM_LE_BREDR_CTRL: c_uint = 0x08 /* Simultaneous LE & BR/EDR Controller */;
pub const LE_AD_SIM_LE_BREDR_HOST: c_uint = 0x10 /* Simultaneous LE & BR/EDR Host */;
// -----  HCI Commands ----
pub const HCI_OP_NOP: c_uint = 0x0000;
pub const HCI_OP_INQUIRY: c_uint = 0x0401;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_inquiry {
    pub lap: [__u8; 3],
    pub length: __u8,
    pub num_rsp: __u8,
    pub __packed: },
pub const HCI_OP_INQUIRY_CANCEL: c_uint = 0x0402;
pub const HCI_OP_PERIODIC_INQ: c_uint = 0x0403;
pub const HCI_OP_EXIT_PERIODIC_INQ: c_uint = 0x0404;
pub const HCI_OP_CREATE_CONN: c_uint = 0x0405;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_create_conn {
    pub bdaddr: bdaddr_t,
    pub pkt_type: __le16,
    pub pscan_rep_mode: __u8,
    pub pscan_mode: __u8,
    pub clock_offset: __le16,
    pub role_switch: __u8,
    pub __packed: },
pub const HCI_OP_DISCONNECT: c_uint = 0x0406;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_disconnect {
    pub handle: __le16,
    pub reason: __u8,
    pub __packed: },
pub const HCI_OP_ADD_SCO: c_uint = 0x0407;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_add_sco {
    pub handle: __le16,
    pub pkt_type: __le16,
    pub __packed: },
pub const HCI_OP_CREATE_CONN_CANCEL: c_uint = 0x0408;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_create_conn_cancel {
    pub bdaddr: bdaddr_t,
    pub __packed: },
pub const HCI_OP_ACCEPT_CONN_REQ: c_uint = 0x0409;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_accept_conn_req {
    pub bdaddr: bdaddr_t,
    pub role: __u8,
    pub __packed: },
pub const HCI_OP_REJECT_CONN_REQ: c_uint = 0x040a;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_reject_conn_req {
    pub bdaddr: bdaddr_t,
    pub reason: __u8,
    pub __packed: },
pub const HCI_OP_LINK_KEY_REPLY: c_uint = 0x040b;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_link_key_reply {
    pub bdaddr: bdaddr_t,
    pub link_key: [__u8; HCI_LINK_KEY_SIZE],
    pub __packed: },
pub const HCI_OP_LINK_KEY_NEG_REPLY: c_uint = 0x040c;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_link_key_neg_reply {
    pub bdaddr: bdaddr_t,
    pub __packed: },
pub const HCI_OP_PIN_CODE_REPLY: c_uint = 0x040d;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_pin_code_reply {
    pub bdaddr: bdaddr_t,
    pub pin_len: __u8,
    pub pin_code: [__u8; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_pin_code_reply {
    pub status: __u8,
    pub bdaddr: bdaddr_t,
    pub __packed: },
pub const HCI_OP_PIN_CODE_NEG_REPLY: c_uint = 0x040e;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_pin_code_neg_reply {
    pub bdaddr: bdaddr_t,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_pin_code_neg_reply {
    pub status: __u8,
    pub bdaddr: bdaddr_t,
    pub __packed: },
pub const HCI_OP_CHANGE_CONN_PTYPE: c_uint = 0x040f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_change_conn_ptype {
    pub handle: __le16,
    pub pkt_type: __le16,
    pub __packed: },
pub const HCI_OP_AUTH_REQUESTED: c_uint = 0x0411;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_auth_requested {
    pub handle: __le16,
    pub __packed: },
pub const HCI_OP_SET_CONN_ENCRYPT: c_uint = 0x0413;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_set_conn_encrypt {
    pub handle: __le16,
    pub encrypt: __u8,
    pub __packed: },
pub const HCI_OP_CHANGE_CONN_LINK_KEY: c_uint = 0x0415;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_change_conn_link_key {
    pub handle: __le16,
    pub __packed: },
pub const HCI_OP_REMOTE_NAME_REQ: c_uint = 0x0419;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_remote_name_req {
    pub bdaddr: bdaddr_t,
    pub pscan_rep_mode: __u8,
    pub pscan_mode: __u8,
    pub clock_offset: __le16,
    pub __packed: },
pub const HCI_OP_REMOTE_NAME_REQ_CANCEL: c_uint = 0x041a;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_remote_name_req_cancel {
    pub bdaddr: bdaddr_t,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_remote_name_req_cancel {
    pub status: __u8,
    pub bdaddr: bdaddr_t,
    pub __packed: },
pub const HCI_OP_READ_REMOTE_FEATURES: c_uint = 0x041b;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_read_remote_features {
    pub handle: __le16,
    pub __packed: },
pub const HCI_OP_READ_REMOTE_EXT_FEATURES: c_uint = 0x041c;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_read_remote_ext_features {
    pub handle: __le16,
    pub page: __u8,
    pub __packed: },
pub const HCI_OP_READ_REMOTE_VERSION: c_uint = 0x041d;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_read_remote_version {
    pub handle: __le16,
    pub __packed: },
pub const HCI_OP_READ_CLOCK_OFFSET: c_uint = 0x041f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_read_clock_offset {
    pub handle: __le16,
    pub __packed: },
pub const HCI_OP_SETUP_SYNC_CONN: c_uint = 0x0428;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_setup_sync_conn {
    pub handle: __le16,
    pub tx_bandwidth: __le32,
    pub rx_bandwidth: __le32,
    pub max_latency: __le16,
    pub voice_setting: __le16,
    pub retrans_effort: __u8,
    pub pkt_type: __le16,
    pub __packed: },
pub const HCI_OP_ACCEPT_SYNC_CONN_REQ: c_uint = 0x0429;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_accept_sync_conn_req {
    pub bdaddr: bdaddr_t,
    pub tx_bandwidth: __le32,
    pub rx_bandwidth: __le32,
    pub max_latency: __le16,
    pub content_format: __le16,
    pub retrans_effort: __u8,
    pub pkt_type: __le16,
    pub __packed: },
pub const HCI_OP_REJECT_SYNC_CONN_REQ: c_uint = 0x042a;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_reject_sync_conn_req {
    pub bdaddr: bdaddr_t,
    pub reason: __u8,
    pub __packed: },
pub const HCI_OP_IO_CAPABILITY_REPLY: c_uint = 0x042b;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_io_capability_reply {
    pub bdaddr: bdaddr_t,
    pub capability: __u8,
    pub oob_data: __u8,
    pub authentication: __u8,
    pub __packed: },
pub const HCI_OP_USER_CONFIRM_REPLY: c_uint = 0x042c;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_user_confirm_reply {
    pub bdaddr: bdaddr_t,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_user_confirm_reply {
    pub status: __u8,
    pub bdaddr: bdaddr_t,
    pub __packed: },
pub const HCI_OP_USER_CONFIRM_NEG_REPLY: c_uint = 0x042d;
pub const HCI_OP_USER_PASSKEY_REPLY: c_uint = 0x042e;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_user_passkey_reply {
    pub bdaddr: bdaddr_t,
    pub passkey: __le32,
    pub __packed: },
pub const HCI_OP_USER_PASSKEY_NEG_REPLY: c_uint = 0x042f;
pub const HCI_OP_REMOTE_OOB_DATA_REPLY: c_uint = 0x0430;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_remote_oob_data_reply {
    pub bdaddr: bdaddr_t,
    pub hash: [__u8; 16],
    pub rand: [__u8; 16],
    pub __packed: },
pub const HCI_OP_REMOTE_OOB_DATA_NEG_REPLY: c_uint = 0x0433;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_remote_oob_data_neg_reply {
    pub bdaddr: bdaddr_t,
    pub __packed: },
pub const HCI_OP_IO_CAPABILITY_NEG_REPLY: c_uint = 0x0434;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_io_capability_neg_reply {
    pub bdaddr: bdaddr_t,
    pub reason: __u8,
    pub __packed: },
pub const HCI_OP_ENHANCED_SETUP_SYNC_CONN: c_uint = 0x043d;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_coding_format {
    pub id: __u8,
    pub cid: __le16,
    pub vid: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_enhanced_setup_sync_conn {
    pub handle: __le16,
    pub tx_bandwidth: __le32,
    pub rx_bandwidth: __le32,
    pub tx_coding_format: hci_coding_format,
    pub rx_coding_format: hci_coding_format,
    pub tx_codec_frame_size: __le16,
    pub rx_codec_frame_size: __le16,
    pub in_bandwidth: __le32,
    pub out_bandwidth: __le32,
    pub in_coding_format: hci_coding_format,
    pub out_coding_format: hci_coding_format,
    pub in_coded_data_size: __le16,
    pub out_coded_data_size: __le16,
    pub in_pcm_data_format: __u8,
    pub out_pcm_data_format: __u8,
    pub in_pcm_sample_payload_msb_pos: __u8,
    pub out_pcm_sample_payload_msb_pos: __u8,
    pub in_data_path: __u8,
    pub out_data_path: __u8,
    pub in_transport_unit_size: __u8,
    pub out_transport_unit_size: __u8,
    pub max_latency: __le16,
    pub pkt_type: __le16,
    pub retrans_effort: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_logical_link_cancel {
    pub status: __u8,
    pub phy_handle: __u8,
    pub flow_spec_id: __u8,
    pub __packed: },
pub const HCI_OP_SET_CPB: c_uint = 0x0441;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_set_cpb {
    pub enable: __u8,
    pub lt_addr: __u8,
    pub lpo_allowed: __u8,
    pub packet_type: __le16,
    pub interval_min: __le16,
    pub interval_max: __le16,
    pub cpb_sv_tout: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_set_cpb {
    pub status: __u8,
    pub lt_addr: __u8,
    pub interval: __le16,
    pub __packed: },
pub const HCI_OP_START_SYNC_TRAIN: c_uint = 0x0443;
pub const HCI_OP_REMOTE_OOB_EXT_DATA_REPLY: c_uint = 0x0445;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_remote_oob_ext_data_reply {
    pub bdaddr: bdaddr_t,
    pub hash192: [__u8; 16],
    pub rand192: [__u8; 16],
    pub hash256: [__u8; 16],
    pub rand256: [__u8; 16],
    pub __packed: },
pub const HCI_OP_SNIFF_MODE: c_uint = 0x0803;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_sniff_mode {
    pub handle: __le16,
    pub max_interval: __le16,
    pub min_interval: __le16,
    pub attempt: __le16,
    pub timeout: __le16,
    pub __packed: },
pub const HCI_OP_EXIT_SNIFF_MODE: c_uint = 0x0804;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_exit_sniff_mode {
    pub handle: __le16,
    pub __packed: },
pub const HCI_OP_ROLE_DISCOVERY: c_uint = 0x0809;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_role_discovery {
    pub handle: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_role_discovery {
    pub status: __u8,
    pub handle: __le16,
    pub role: __u8,
    pub __packed: },
pub const HCI_OP_SWITCH_ROLE: c_uint = 0x080b;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_switch_role {
    pub bdaddr: bdaddr_t,
    pub role: __u8,
    pub __packed: },
pub const HCI_OP_READ_LINK_POLICY: c_uint = 0x080c;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_read_link_policy {
    pub handle: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_link_policy {
    pub status: __u8,
    pub handle: __le16,
    pub policy: __le16,
    pub __packed: },
pub const HCI_OP_WRITE_LINK_POLICY: c_uint = 0x080d;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_write_link_policy {
    pub handle: __le16,
    pub policy: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_write_link_policy {
    pub status: __u8,
    pub handle: __le16,
    pub __packed: },
pub const HCI_OP_READ_DEF_LINK_POLICY: c_uint = 0x080e;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_def_link_policy {
    pub status: __u8,
    pub policy: __le16,
    pub __packed: },
pub const HCI_OP_WRITE_DEF_LINK_POLICY: c_uint = 0x080f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_write_def_link_policy {
    pub policy: __le16,
    pub __packed: },
pub const HCI_OP_SNIFF_SUBRATE: c_uint = 0x0811;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_sniff_subrate {
    pub handle: __le16,
    pub max_latency: __le16,
    pub min_remote_timeout: __le16,
    pub min_local_timeout: __le16,
    pub __packed: },
pub const HCI_OP_SET_EVENT_MASK: c_uint = 0x0c01;
pub const HCI_OP_RESET: c_uint = 0x0c03;
pub const HCI_OP_SET_EVENT_FLT: c_uint = 0x0c05;
pub const HCI_SET_EVENT_FLT_SIZE: c_int = 9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_set_event_filter {
    pub flt_type: __u8,
    pub cond_type: __u8,
    pub bdaddr: bdaddr_t,
    pub auto_accept: __u8,
    pub addr_conn_flt: } __packed,
    pub __packed: },
// Filter types
pub const HCI_FLT_CLEAR_ALL: c_uint = 0x00;
pub const HCI_FLT_INQ_RESULT: c_uint = 0x01;
pub const HCI_FLT_CONN_SETUP: c_uint = 0x02;
// CONN_SETUP Condition types
pub const HCI_CONN_SETUP_ALLOW_ALL: c_uint = 0x00;
pub const HCI_CONN_SETUP_ALLOW_CLASS: c_uint = 0x01;
pub const HCI_CONN_SETUP_ALLOW_BDADDR: c_uint = 0x02;
// CONN_SETUP Conditions
pub const HCI_CONN_SETUP_AUTO_OFF: c_uint = 0x01;
pub const HCI_CONN_SETUP_AUTO_ON: c_uint = 0x02;
pub const HCI_CONN_SETUP_AUTO_ON_WITH_RS: c_uint = 0x03;
pub const HCI_OP_READ_STORED_LINK_KEY: c_uint = 0x0c0d;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_read_stored_link_key {
    pub bdaddr: bdaddr_t,
    pub read_all: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_stored_link_key {
    pub status: __u8,
    pub max_keys: __le16,
    pub num_keys: __le16,
    pub __packed: },
pub const HCI_OP_DELETE_STORED_LINK_KEY: c_uint = 0x0c12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_delete_stored_link_key {
    pub bdaddr: bdaddr_t,
    pub delete_all: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_delete_stored_link_key {
    pub status: __u8,
    pub num_keys: __le16,
    pub __packed: },
pub const HCI_MAX_NAME_LENGTH: c_int = 248;
pub const HCI_OP_WRITE_LOCAL_NAME: c_uint = 0x0c13;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_write_local_name {
    pub name: [__u8; HCI_MAX_NAME_LENGTH],
    pub __packed: },
pub const HCI_OP_READ_LOCAL_NAME: c_uint = 0x0c14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_local_name {
    pub status: __u8,
    pub name: [__u8; HCI_MAX_NAME_LENGTH],
    pub __packed: },
pub const HCI_OP_WRITE_CA_TIMEOUT: c_uint = 0x0c16;
pub const HCI_OP_WRITE_PG_TIMEOUT: c_uint = 0x0c18;
pub const HCI_OP_WRITE_SCAN_ENABLE: c_uint = 0x0c1a;
pub const SCAN_DISABLED: c_uint = 0x00;
pub const SCAN_INQUIRY: c_uint = 0x01;
pub const SCAN_PAGE: c_uint = 0x02;
pub const HCI_OP_READ_AUTH_ENABLE: c_uint = 0x0c1f;
pub const HCI_OP_WRITE_AUTH_ENABLE: c_uint = 0x0c20;
pub const AUTH_DISABLED: c_uint = 0x00;
pub const AUTH_ENABLED: c_uint = 0x01;
pub const HCI_OP_READ_ENCRYPT_MODE: c_uint = 0x0c21;
pub const HCI_OP_WRITE_ENCRYPT_MODE: c_uint = 0x0c22;
pub const ENCRYPT_DISABLED: c_uint = 0x00;
pub const ENCRYPT_P2P: c_uint = 0x01;
pub const ENCRYPT_BOTH: c_uint = 0x02;
pub const HCI_OP_READ_CLASS_OF_DEV: c_uint = 0x0c23;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_class_of_dev {
    pub status: __u8,
    pub dev_class: [__u8; 3],
    pub __packed: },
pub const HCI_OP_WRITE_CLASS_OF_DEV: c_uint = 0x0c24;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_write_class_of_dev {
    pub dev_class: [__u8; 3],
    pub __packed: },
pub const HCI_OP_READ_VOICE_SETTING: c_uint = 0x0c25;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_voice_setting {
    pub status: __u8,
    pub voice_setting: __le16,
    pub __packed: },
pub const HCI_OP_WRITE_VOICE_SETTING: c_uint = 0x0c26;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_write_voice_setting {
    pub voice_setting: __le16,
    pub __packed: },
pub const HCI_OP_HOST_BUFFER_SIZE: c_uint = 0x0c33;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_host_buffer_size {
    pub acl_mtu: __le16,
    pub sco_mtu: __u8,
    pub acl_max_pkt: __le16,
    pub sco_max_pkt: __le16,
    pub __packed: },
pub const HCI_OP_READ_NUM_SUPPORTED_IAC: c_uint = 0x0c38;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_num_supported_iac {
    pub status: __u8,
    pub num_iac: __u8,
    pub __packed: },
pub const HCI_OP_READ_CURRENT_IAC_LAP: c_uint = 0x0c39;
pub const HCI_OP_WRITE_CURRENT_IAC_LAP: c_uint = 0x0c3a;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_write_current_iac_lap {
    pub num_iac: __u8,
    pub iac_lap: [__u8; 6],
    pub __packed: },
pub const HCI_OP_WRITE_INQUIRY_MODE: c_uint = 0x0c45;
pub const HCI_MAX_EIR_LENGTH: c_int = 240;
pub const HCI_OP_WRITE_EIR: c_uint = 0x0c52;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_write_eir {
    pub fec: __u8,
    pub data: [__u8; HCI_MAX_EIR_LENGTH],
    pub __packed: },
pub const HCI_OP_READ_SSP_MODE: c_uint = 0x0c55;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_ssp_mode {
    pub status: __u8,
    pub mode: __u8,
    pub __packed: },
pub const HCI_OP_WRITE_SSP_MODE: c_uint = 0x0c56;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_write_ssp_mode {
    pub mode: __u8,
    pub __packed: },
pub const HCI_OP_READ_LOCAL_OOB_DATA: c_uint = 0x0c57;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_local_oob_data {
    pub status: __u8,
    pub hash: [__u8; 16],
    pub rand: [__u8; 16],
    pub __packed: },
pub const HCI_OP_READ_INQ_RSP_TX_POWER: c_uint = 0x0c58;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_inq_rsp_tx_power {
    pub status: __u8,
    pub tx_power: __s8,
    pub __packed: },
pub const HCI_OP_READ_DEF_ERR_DATA_REPORTING: c_uint = 0x0c5a;
pub const ERR_DATA_REPORTING_DISABLED: c_uint = 0x00;
pub const ERR_DATA_REPORTING_ENABLED: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_def_err_data_reporting {
    pub status: __u8,
    pub err_data_reporting: __u8,
    pub __packed: },
pub const HCI_OP_WRITE_DEF_ERR_DATA_REPORTING: c_uint = 0x0c5b;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_write_def_err_data_reporting {
    pub err_data_reporting: __u8,
    pub __packed: },
pub const HCI_OP_SET_EVENT_MASK_PAGE_2: c_uint = 0x0c63;
pub const HCI_OP_READ_LOCATION_DATA: c_uint = 0x0c64;
pub const HCI_OP_READ_FLOW_CONTROL_MODE: c_uint = 0x0c66;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_flow_control_mode {
    pub status: __u8,
    pub mode: __u8,
    pub __packed: },
pub const HCI_OP_WRITE_LE_HOST_SUPPORTED: c_uint = 0x0c6d;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_write_le_host_supported {
    pub le: __u8,
    pub simul: __u8,
    pub __packed: },
pub const HCI_OP_SET_RESERVED_LT_ADDR: c_uint = 0x0c74;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_set_reserved_lt_addr {
    pub lt_addr: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_set_reserved_lt_addr {
    pub status: __u8,
    pub lt_addr: __u8,
    pub __packed: },
pub const HCI_OP_DELETE_RESERVED_LT_ADDR: c_uint = 0x0c75;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_delete_reserved_lt_addr {
    pub lt_addr: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_delete_reserved_lt_addr {
    pub status: __u8,
    pub lt_addr: __u8,
    pub __packed: },
pub const HCI_OP_SET_CPB_DATA: c_uint = 0x0c76;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_set_cpb_data {
    pub lt_addr: __u8,
    pub fragment: __u8,
    pub data_length: __u8,
    pub data: [__u8; HCI_MAX_CPB_DATA_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_set_cpb_data {
    pub status: __u8,
    pub lt_addr: __u8,
    pub __packed: },
pub const HCI_OP_READ_SYNC_TRAIN_PARAMS: c_uint = 0x0c77;
pub const HCI_OP_WRITE_SYNC_TRAIN_PARAMS: c_uint = 0x0c78;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_write_sync_train_params {
    pub interval_min: __le16,
    pub interval_max: __le16,
    pub sync_train_tout: __le32,
    pub service_data: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_write_sync_train_params {
    pub status: __u8,
    pub sync_train_int: __le16,
    pub __packed: },
pub const HCI_OP_READ_SC_SUPPORT: c_uint = 0x0c79;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_sc_support {
    pub status: __u8,
    pub support: __u8,
    pub __packed: },
pub const HCI_OP_WRITE_SC_SUPPORT: c_uint = 0x0c7a;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_write_sc_support {
    pub support: __u8,
    pub __packed: },
pub const HCI_OP_READ_AUTH_PAYLOAD_TO: c_uint = 0x0c7b;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_read_auth_payload_to {
    pub handle: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_auth_payload_to {
    pub status: __u8,
    pub handle: __le16,
    pub timeout: __le16,
    pub __packed: },
pub const HCI_OP_WRITE_AUTH_PAYLOAD_TO: c_uint = 0x0c7c;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_write_auth_payload_to {
    pub handle: __le16,
    pub timeout: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_write_auth_payload_to {
    pub status: __u8,
    pub handle: __le16,
    pub __packed: },
pub const HCI_OP_READ_LOCAL_OOB_EXT_DATA: c_uint = 0x0c7d;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_local_oob_ext_data {
    pub status: __u8,
    pub hash192: [__u8; 16],
    pub rand192: [__u8; 16],
    pub hash256: [__u8; 16],
    pub rand256: [__u8; 16],
    pub __packed: },
pub const HCI_CONFIGURE_DATA_PATH: c_uint = 0x0c83;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_op_configure_data_path {
    pub direction: __u8,
    pub data_path_id: __u8,
    pub vnd_len: __u8,
    pub vnd_data: [__u8; ],
    pub __packed: },
pub const HCI_OP_READ_LOCAL_VERSION: c_uint = 0x1001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_local_version {
    pub status: __u8,
    pub hci_ver: __u8,
    pub hci_rev: __le16,
    pub lmp_ver: __u8,
    pub manufacturer: __le16,
    pub lmp_subver: __le16,
    pub __packed: },
pub const HCI_OP_READ_LOCAL_COMMANDS: c_uint = 0x1002;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_local_commands {
    pub status: __u8,
    pub commands: [__u8; 64],
    pub __packed: },
pub const HCI_OP_READ_LOCAL_FEATURES: c_uint = 0x1003;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_local_features {
    pub status: __u8,
    pub features: [__u8; 8],
    pub __packed: },
pub const HCI_OP_READ_LOCAL_EXT_FEATURES: c_uint = 0x1004;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_read_local_ext_features {
    pub page: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_local_ext_features {
    pub status: __u8,
    pub page: __u8,
    pub max_page: __u8,
    pub features: [__u8; 8],
    pub __packed: },
pub const HCI_OP_READ_BUFFER_SIZE: c_uint = 0x1005;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_buffer_size {
    pub status: __u8,
    pub acl_mtu: __le16,
    pub sco_mtu: __u8,
    pub acl_max_pkt: __le16,
    pub sco_max_pkt: __le16,
    pub __packed: },
pub const HCI_OP_READ_BD_ADDR: c_uint = 0x1009;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_bd_addr {
    pub status: __u8,
    pub bdaddr: bdaddr_t,
    pub __packed: },
pub const HCI_OP_READ_DATA_BLOCK_SIZE: c_uint = 0x100a;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_data_block_size {
    pub status: __u8,
    pub max_acl_len: __le16,
    pub block_len: __le16,
    pub num_blocks: __le16,
    pub __packed: },
pub const HCI_OP_READ_LOCAL_CODECS: c_uint = 0x100b;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_std_codecs_hdr {
    pub num: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_std_codecs {
    pub hci_std_codecs_hdr: struct,
    pub codec: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_vnd_codec {
// company id
    pub cid: __le16,
// vendor codec id
    pub vid: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_vnd_codecs {
    pub num: __u8,
    pub codec: [hci_vnd_codec; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_local_supported_codecs {
    pub status: __u8,
    pub std_codecs: hci_std_codecs_hdr,
    pub vnd_codecs: hci_vnd_codecs,
    pub __packed: },
pub const HCI_OP_READ_LOCAL_PAIRING_OPTS: c_uint = 0x100c;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_local_pairing_opts {
    pub status: __u8,
    pub pairing_opts: __u8,
    pub max_key_size: __u8,
    pub __packed: },
pub const HCI_OP_READ_LOCAL_CODECS_V2: c_uint = 0x100d;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_std_codec_v2 {
    pub id: __u8,
    pub transport: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_std_codecs_v2_hdr {
    pub num: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_std_codecs_v2 {
    pub hci_std_codecs_v2_hdr: struct,
    pub codec: [hci_std_codec_v2; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_vnd_codec_v2 {
    pub cid: __le16,
    pub vid: __le16,
    pub transport: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_vnd_codecs_v2 {
    pub num: __u8,
    pub codec: [hci_vnd_codec_v2; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_local_supported_codecs_v2 {
    pub status: __u8,
    pub std_codecs: hci_std_codecs_v2_hdr,
    pub vendor_codecs: hci_vnd_codecs_v2,
    pub __packed: },
pub const HCI_OP_READ_LOCAL_CODEC_CAPS: c_uint = 0x100e;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_op_read_local_codec_caps {
    pub id: __u8,
    pub cid: __le16,
    pub vid: __le16,
    pub transport: __u8,
    pub direction: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_codec_caps {
    pub len: __u8,
    pub data: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_local_codec_caps {
    pub status: __u8,
    pub num_caps: __u8,
    pub __packed: },
pub const HCI_OP_READ_PAGE_SCAN_ACTIVITY: c_uint = 0x0c1b;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_page_scan_activity {
    pub status: __u8,
    pub interval: __le16,
    pub window: __le16,
    pub __packed: },
pub const HCI_OP_WRITE_PAGE_SCAN_ACTIVITY: c_uint = 0x0c1c;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_write_page_scan_activity {
    pub interval: __le16,
    pub window: __le16,
    pub __packed: },
pub const HCI_OP_READ_TX_POWER: c_uint = 0x0c2d;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_read_tx_power {
    pub handle: __le16,
    pub type: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_tx_power {
    pub status: __u8,
    pub handle: __le16,
    pub tx_power: __s8,
    pub __packed: },
pub const HCI_OP_WRITE_SYNC_FLOWCTL: c_uint = 0x0c2f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_write_sync_flowctl {
    pub enable: __u8,
    pub __packed: },
pub const HCI_OP_READ_PAGE_SCAN_TYPE: c_uint = 0x0c46;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_page_scan_type {
    pub status: __u8,
    pub type: __u8,
    pub __packed: },
pub const HCI_OP_WRITE_PAGE_SCAN_TYPE: c_uint = 0x0c47;
pub const PAGE_SCAN_TYPE_STANDARD: c_uint = 0x00;
pub const PAGE_SCAN_TYPE_INTERLACED: c_uint = 0x01;
pub const HCI_OP_READ_RSSI: c_uint = 0x1405;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_read_rssi {
    pub handle: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_rssi {
    pub status: __u8,
    pub handle: __le16,
    pub rssi: __s8,
    pub __packed: },
pub const HCI_OP_READ_CLOCK: c_uint = 0x1407;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_read_clock {
    pub handle: __le16,
    pub which: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_clock {
    pub status: __u8,
    pub handle: __le16,
    pub clock: __le32,
    pub accuracy: __le16,
    pub __packed: },
pub const HCI_OP_READ_ENC_KEY_SIZE: c_uint = 0x1408;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_read_enc_key_size {
    pub handle: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_read_enc_key_size {
    pub status: __u8,
    pub handle: __le16,
    pub key_size: __u8,
    pub __packed: },
pub const HCI_OP_GET_MWS_TRANSPORT_CONFIG: c_uint = 0x140c;
pub const HCI_OP_ENABLE_DUT_MODE: c_uint = 0x1803;
pub const HCI_OP_WRITE_SSP_DEBUG_MODE: c_uint = 0x1804;
pub const HCI_OP_LE_SET_EVENT_MASK: c_uint = 0x2001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_set_event_mask {
    pub mask: [__u8; 8],
    pub __packed: },
// BLUETOOTH CORE SPECIFICATION Version 5.4 | Vol 4, Part E
// 7.8.2 LE Read Buffer Size command
// MAX_LE_MTU is 0xffff.
// 0 is also valid. It means that no dedicated LE Buffer exists.
// It should use the HCI_Read_Buffer_Size command and mtu is shared
// between BR/EDR and LE.
//
pub const HCI_MIN_LE_MTU: c_uint = 0x001b;
pub const HCI_OP_LE_READ_BUFFER_SIZE: c_uint = 0x2002;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_le_read_buffer_size {
    pub status: __u8,
    pub le_mtu: __le16,
    pub le_max_pkt: __u8,
    pub __packed: },
pub const HCI_OP_LE_READ_LOCAL_FEATURES: c_uint = 0x2003;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_le_read_local_features {
    pub status: __u8,
    pub features: [__u8; 8],
    pub __packed: },
pub const HCI_OP_LE_SET_RANDOM_ADDR: c_uint = 0x2005;
pub const HCI_OP_LE_SET_ADV_PARAM: c_uint = 0x2006;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_set_adv_param {
    pub min_interval: __le16,
    pub max_interval: __le16,
    pub type: __u8,
    pub own_address_type: __u8,
    pub direct_addr_type: __u8,
    pub direct_addr: bdaddr_t,
    pub channel_map: __u8,
    pub filter_policy: __u8,
    pub __packed: },
pub const HCI_OP_LE_READ_ADV_TX_POWER: c_uint = 0x2007;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_le_read_adv_tx_power {
    pub status: __u8,
    pub tx_power: __s8,
    pub __packed: },
pub const HCI_MAX_AD_LENGTH: c_int = 31;
pub const HCI_OP_LE_SET_ADV_DATA: c_uint = 0x2008;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_set_adv_data {
    pub length: __u8,
    pub data: [__u8; HCI_MAX_AD_LENGTH],
    pub __packed: },
pub const HCI_OP_LE_SET_SCAN_RSP_DATA: c_uint = 0x2009;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_set_scan_rsp_data {
    pub length: __u8,
    pub data: [__u8; HCI_MAX_AD_LENGTH],
    pub __packed: },
pub const HCI_OP_LE_SET_ADV_ENABLE: c_uint = 0x200a;
pub const LE_SCAN_PASSIVE: c_uint = 0x00;
pub const LE_SCAN_ACTIVE: c_uint = 0x01;
pub const HCI_OP_LE_SET_SCAN_PARAM: c_uint = 0x200b;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_set_scan_param {
    pub type: __u8,
    pub interval: __le16,
    pub window: __le16,
    pub own_address_type: __u8,
    pub filter_policy: __u8,
    pub __packed: },
pub const LE_SCAN_DISABLE: c_uint = 0x00;
pub const LE_SCAN_ENABLE: c_uint = 0x01;
pub const LE_SCAN_FILTER_DUP_DISABLE: c_uint = 0x00;
pub const LE_SCAN_FILTER_DUP_ENABLE: c_uint = 0x01;
pub const HCI_OP_LE_SET_SCAN_ENABLE: c_uint = 0x200c;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_set_scan_enable {
    pub enable: __u8,
    pub filter_dup: __u8,
    pub __packed: },
pub const HCI_LE_USE_PEER_ADDR: c_uint = 0x00;
pub const HCI_LE_USE_ACCEPT_LIST: c_uint = 0x01;
pub const HCI_OP_LE_CREATE_CONN: c_uint = 0x200d;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_create_conn {
    pub scan_interval: __le16,
    pub scan_window: __le16,
    pub filter_policy: __u8,
    pub peer_addr_type: __u8,
    pub peer_addr: bdaddr_t,
    pub own_address_type: __u8,
    pub conn_interval_min: __le16,
    pub conn_interval_max: __le16,
    pub conn_latency: __le16,
    pub supervision_timeout: __le16,
    pub min_ce_len: __le16,
    pub max_ce_len: __le16,
    pub __packed: },
pub const HCI_OP_LE_CREATE_CONN_CANCEL: c_uint = 0x200e;
pub const HCI_OP_LE_READ_ACCEPT_LIST_SIZE: c_uint = 0x200f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_le_read_accept_list_size {
    pub status: __u8,
    pub size: __u8,
    pub __packed: },
pub const HCI_OP_LE_CLEAR_ACCEPT_LIST: c_uint = 0x2010;
pub const HCI_OP_LE_ADD_TO_ACCEPT_LIST: c_uint = 0x2011;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_add_to_accept_list {
    pub bdaddr_type: __u8,
    pub bdaddr: bdaddr_t,
    pub __packed: },
pub const HCI_OP_LE_DEL_FROM_ACCEPT_LIST: c_uint = 0x2012;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_del_from_accept_list {
    pub bdaddr_type: __u8,
    pub bdaddr: bdaddr_t,
    pub __packed: },
pub const HCI_OP_LE_CONN_UPDATE: c_uint = 0x2013;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_conn_update {
    pub handle: __le16,
    pub conn_interval_min: __le16,
    pub conn_interval_max: __le16,
    pub conn_latency: __le16,
    pub supervision_timeout: __le16,
    pub min_ce_len: __le16,
    pub max_ce_len: __le16,
    pub __packed: },
pub const HCI_OP_LE_READ_REMOTE_FEATURES: c_uint = 0x2016;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_read_remote_features {
    pub handle: __le16,
    pub __packed: },
pub const HCI_OP_LE_START_ENC: c_uint = 0x2019;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_start_enc {
    pub handle: __le16,
    pub rand: __le64,
    pub ediv: __le16,
    pub ltk: [__u8; 16],
    pub __packed: },
pub const HCI_OP_LE_LTK_REPLY: c_uint = 0x201a;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_ltk_reply {
    pub handle: __le16,
    pub ltk: [__u8; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_le_ltk_reply {
    pub status: __u8,
    pub handle: __le16,
    pub __packed: },
pub const HCI_OP_LE_LTK_NEG_REPLY: c_uint = 0x201b;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_ltk_neg_reply {
    pub handle: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_le_ltk_neg_reply {
    pub status: __u8,
    pub handle: __le16,
    pub __packed: },
pub const HCI_OP_LE_READ_SUPPORTED_STATES: c_uint = 0x201c;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_le_read_supported_states {
    pub status: __u8,
    pub le_states: [__u8; 8],
    pub __packed: },
pub const HCI_OP_LE_CONN_PARAM_REQ_REPLY: c_uint = 0x2020;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_conn_param_req_reply {
    pub handle: __le16,
    pub interval_min: __le16,
    pub interval_max: __le16,
    pub latency: __le16,
    pub timeout: __le16,
    pub min_ce_len: __le16,
    pub max_ce_len: __le16,
    pub __packed: },
pub const HCI_OP_LE_CONN_PARAM_REQ_NEG_REPLY: c_uint = 0x2021;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_conn_param_req_neg_reply {
    pub handle: __le16,
    pub reason: __u8,
    pub __packed: },
pub const HCI_OP_LE_SET_DATA_LEN: c_uint = 0x2022;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_set_data_len {
    pub handle: __le16,
    pub tx_len: __le16,
    pub tx_time: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_le_set_data_len {
    pub status: __u8,
    pub handle: __le16,
    pub __packed: },
pub const HCI_OP_LE_READ_DEF_DATA_LEN: c_uint = 0x2023;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_le_read_def_data_len {
    pub status: __u8,
    pub tx_len: __le16,
    pub tx_time: __le16,
    pub __packed: },
pub const HCI_OP_LE_WRITE_DEF_DATA_LEN: c_uint = 0x2024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_write_def_data_len {
    pub tx_len: __le16,
    pub tx_time: __le16,
    pub __packed: },
pub const HCI_OP_LE_ADD_TO_RESOLV_LIST: c_uint = 0x2027;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_add_to_resolv_list {
    pub bdaddr_type: __u8,
    pub bdaddr: bdaddr_t,
    pub peer_irk: [__u8; 16],
    pub local_irk: [__u8; 16],
    pub __packed: },
pub const HCI_OP_LE_DEL_FROM_RESOLV_LIST: c_uint = 0x2028;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_del_from_resolv_list {
    pub bdaddr_type: __u8,
    pub bdaddr: bdaddr_t,
    pub __packed: },
pub const HCI_OP_LE_CLEAR_RESOLV_LIST: c_uint = 0x2029;
pub const HCI_OP_LE_READ_RESOLV_LIST_SIZE: c_uint = 0x202a;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_le_read_resolv_list_size {
    pub status: __u8,
    pub size: __u8,
    pub __packed: },
pub const HCI_OP_LE_SET_ADDR_RESOLV_ENABLE: c_uint = 0x202d;
pub const HCI_OP_LE_SET_RPA_TIMEOUT: c_uint = 0x202e;
pub const HCI_OP_LE_READ_MAX_DATA_LEN: c_uint = 0x202f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_le_read_max_data_len {
    pub status: __u8,
    pub tx_len: __le16,
    pub tx_time: __le16,
    pub rx_len: __le16,
    pub rx_time: __le16,
    pub __packed: },
pub const HCI_OP_LE_SET_DEFAULT_PHY: c_uint = 0x2031;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_set_default_phy {
    pub all_phys: __u8,
    pub tx_phys: __u8,
    pub rx_phys: __u8,
    pub __packed: },
pub const HCI_LE_SET_PHY_1M: c_uint = 0x01;
pub const HCI_LE_SET_PHY_2M: c_uint = 0x02;
pub const HCI_LE_SET_PHY_CODED: c_uint = 0x04;
pub const HCI_OP_LE_SET_PHY: c_uint = 0x2032;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_set_phy {
    pub handle: __le16,
    pub all_phys: __u8,
    pub tx_phys: __u8,
    pub rx_phys: __u8,
    pub phy_opts: __le16,
    pub __packed: },
pub const HCI_OP_LE_SET_EXT_SCAN_PARAMS: c_uint = 0x2041;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_set_ext_scan_params {
    pub own_addr_type: __u8,
    pub filter_policy: __u8,
    pub scanning_phys: __u8,
    pub data: [__u8; ],
    pub __packed: },
pub const LE_SCAN_PHY_1M: c_uint = 0x01;
pub const LE_SCAN_PHY_2M: c_uint = 0x02;
pub const LE_SCAN_PHY_CODED: c_uint = 0x04;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_scan_phy_params {
    pub type: __u8,
    pub interval: __le16,
    pub window: __le16,
    pub __packed: },
pub const HCI_OP_LE_SET_EXT_SCAN_ENABLE: c_uint = 0x2042;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_set_ext_scan_enable {
    pub enable: __u8,
    pub filter_dup: __u8,
    pub duration: __le16,
    pub period: __le16,
    pub __packed: },
pub const HCI_OP_LE_EXT_CREATE_CONN: c_uint = 0x2043;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_ext_create_conn {
    pub filter_policy: __u8,
    pub own_addr_type: __u8,
    pub peer_addr_type: __u8,
    pub peer_addr: bdaddr_t,
    pub phys: __u8,
    pub data: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_ext_conn_param {
    pub scan_interval: __le16,
    pub scan_window: __le16,
    pub conn_interval_min: __le16,
    pub conn_interval_max: __le16,
    pub conn_latency: __le16,
    pub supervision_timeout: __le16,
    pub min_ce_len: __le16,
    pub max_ce_len: __le16,
    pub __packed: },
pub const HCI_OP_LE_PA_CREATE_SYNC: c_uint = 0x2044;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_pa_create_sync {
    pub options: __u8,
    pub sid: __u8,
    pub addr_type: __u8,
    pub addr: bdaddr_t,
    pub skip: __le16,
    pub sync_timeout: __le16,
    pub sync_cte_type: __u8,
    pub __packed: },
pub const HCI_OP_LE_PA_CREATE_SYNC_CANCEL: c_uint = 0x2045;
pub const HCI_OP_LE_PA_TERM_SYNC: c_uint = 0x2046;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_pa_term_sync {
    pub handle: __le16,
    pub __packed: },
pub const HCI_OP_LE_READ_NUM_SUPPORTED_ADV_SETS: c_uint = 0x203b;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_le_read_num_supported_adv_sets {
    pub status: __u8,
    pub num_of_sets: __u8,
    pub __packed: },
pub const HCI_OP_LE_SET_EXT_ADV_PARAMS: c_uint = 0x2036;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_set_ext_adv_params {
    pub handle: __u8,
    pub evt_properties: __le16,
    pub min_interval: [__u8; 3],
    pub max_interval: [__u8; 3],
    pub channel_map: __u8,
    pub own_addr_type: __u8,
    pub peer_addr_type: __u8,
    pub peer_addr: bdaddr_t,
    pub filter_policy: __u8,
    pub tx_power: __u8,
    pub primary_phy: __u8,
    pub secondary_max_skip: __u8,
    pub secondary_phy: __u8,
    pub sid: __u8,
    pub notif_enable: __u8,
    pub __packed: },

pub const HCI_ADV_PHY_2M: c_uint = 0x02;
pub const HCI_ADV_PHY_CODED: c_uint = 0x03;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_le_set_ext_adv_params {
    pub status: __u8,
    pub tx_power: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_ext_adv_set {
    pub handle: __u8,
    pub duration: __le16,
    pub max_events: __u8,
    pub __packed: },
pub const HCI_MAX_EXT_AD_LENGTH: c_int = 251;
pub const HCI_OP_LE_SET_EXT_ADV_DATA: c_uint = 0x2037;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_set_ext_adv_data {
    pub handle: __u8,
    pub operation: __u8,
    pub frag_pref: __u8,
    pub length: __u8,
    pub __counted_by(length): __u8 data[],
    pub __packed: },
pub const HCI_OP_LE_SET_EXT_SCAN_RSP_DATA: c_uint = 0x2038;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_set_ext_scan_rsp_data {
    pub handle: __u8,
    pub operation: __u8,
    pub frag_pref: __u8,
    pub length: __u8,
    pub __counted_by(length): __u8 data[],
    pub __packed: },
pub const HCI_OP_LE_SET_EXT_ADV_ENABLE: c_uint = 0x2039;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_set_ext_adv_enable {
    pub enable: __u8,
    pub num_of_sets: __u8,
    pub data: [__u8; ],
    pub __packed: },
pub const HCI_OP_LE_SET_PER_ADV_PARAMS: c_uint = 0x203e;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_set_per_adv_params {
    pub handle: __u8,
    pub min_interval: __le16,
    pub max_interval: __le16,
    pub periodic_properties: __le16,
    pub __packed: },
pub const HCI_MAX_PER_AD_LENGTH: c_int = 252;
pub const HCI_MAX_PER_AD_TOT_LEN: c_int = 1650;
pub const HCI_OP_LE_SET_PER_ADV_DATA: c_uint = 0x203f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_set_per_adv_data {
    pub handle: __u8,
    pub operation: __u8,
    pub length: __u8,
    pub __counted_by(length): __u8 data[],
    pub __packed: },
pub const HCI_OP_LE_SET_PER_ADV_ENABLE: c_uint = 0x2040;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_set_per_adv_enable {
    pub enable: __u8,
    pub handle: __u8,
    pub __packed: },
pub const LE_SET_ADV_DATA_OP_COMPLETE: c_uint = 0x03;
pub const LE_SET_ADV_DATA_NO_FRAG: c_uint = 0x01;
pub const HCI_OP_LE_REMOVE_ADV_SET: c_uint = 0x203c;
pub const HCI_OP_LE_CLEAR_ADV_SETS: c_uint = 0x203d;
pub const HCI_OP_LE_SET_ADV_SET_RAND_ADDR: c_uint = 0x2035;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_set_adv_set_rand_addr {
    pub handle: __u8,
    pub bdaddr: bdaddr_t,
    pub __packed: },
pub const HCI_OP_LE_READ_TRANSMIT_POWER: c_uint = 0x204b;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_le_read_transmit_power {
    pub status: __u8,
    pub min_le_tx_power: __s8,
    pub max_le_tx_power: __s8,
    pub __packed: },
pub const HCI_NETWORK_PRIVACY: c_uint = 0x00;
pub const HCI_DEVICE_PRIVACY: c_uint = 0x01;
pub const HCI_OP_LE_SET_PRIVACY_MODE: c_uint = 0x204e;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_set_privacy_mode {
    pub bdaddr_type: __u8,
    pub bdaddr: bdaddr_t,
    pub mode: __u8,
    pub __packed: },
pub const HCI_OP_LE_PAST: c_uint = 0x205a;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_past {
    pub handle: __le16,
    pub service_data: __le16,
    pub sync_handle: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_le_past {
    pub status: __u8,
    pub handle: __le16,
    pub __packed: },
pub const HCI_OP_LE_PAST_SET_INFO: c_uint = 0x205b;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_past_set_info {
    pub handle: __le16,
    pub service_data: __le16,
    pub adv_handle: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_le_past_set_info {
    pub status: __u8,
    pub handle: __le16,
    pub __packed: },
pub const HCI_OP_LE_PAST_PARAMS: c_uint = 0x205c;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_past_params {
    pub handle: __le16,
    pub mode: __u8,
    pub skip: __le16,
    pub sync_timeout: __le16,
    pub cte_type: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_le_past_params {
    pub status: __u8,
    pub handle: __le16,
    pub __packed: },
pub const HCI_OP_LE_READ_BUFFER_SIZE_V2: c_uint = 0x2060;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_le_read_buffer_size_v2 {
    pub status: __u8,
    pub acl_mtu: __le16,
    pub acl_max_pkt: __u8,
    pub iso_mtu: __le16,
    pub iso_max_pkt: __u8,
    pub __packed: },
pub const HCI_OP_LE_READ_ISO_TX_SYNC: c_uint = 0x2061;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_read_iso_tx_sync {
    pub handle: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_le_read_iso_tx_sync {
    pub status: __u8,
    pub handle: __le16,
    pub seq: __le16,
    pub imestamp: __le32,
    pub offset: [__u8; 3],
    pub __packed: },
pub const HCI_OP_LE_SET_CIG_PARAMS: c_uint = 0x2062;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cis_params {
    pub cis_id: __u8,
    pub c_sdu: __le16,
    pub p_sdu: __le16,
    pub c_phys: __u8,
    pub p_phys: __u8,
    pub c_rtn: __u8,
    pub p_rtn: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_set_cig_params {
    pub cig_id: __u8,
    pub c_interval: [__u8; 3],
    pub p_interval: [__u8; 3],
    pub sca: __u8,
    pub packing: __u8,
    pub framing: __u8,
    pub c_latency: __le16,
    pub p_latency: __le16,
    pub num_cis: __u8,
    pub __counted_by(num_cis): hci_cis_params cis[],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_le_set_cig_params {
    pub status: __u8,
    pub cig_id: __u8,
    pub num_handles: __u8,
    pub handle: [__le16; ],
    pub __packed: },
pub const HCI_OP_LE_CREATE_CIS: c_uint = 0x2064;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cis {
    pub cis_handle: __le16,
    pub acl_handle: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_create_cis {
    pub num_cis: __u8,
    pub __counted_by(num_cis): hci_cis cis[],
    pub __packed: },
pub const HCI_OP_LE_REMOVE_CIG: c_uint = 0x2065;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_remove_cig {
    pub cig_id: __u8,
    pub __packed: },
pub const HCI_OP_LE_ACCEPT_CIS: c_uint = 0x2066;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_accept_cis {
    pub handle: __le16,
    pub __packed: },
pub const HCI_OP_LE_REJECT_CIS: c_uint = 0x2067;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_reject_cis {
    pub handle: __le16,
    pub reason: __u8,
    pub __packed: },
pub const HCI_OP_LE_CREATE_BIG: c_uint = 0x2068;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_bis {
    pub sdu_interval: [__u8; 3],
    pub sdu: __le16,
    pub latency: __le16,
    pub rtn: __u8,
    pub phy: __u8,
    pub packing: __u8,
    pub framing: __u8,
    pub encryption: __u8,
    pub bcode: [__u8; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_create_big {
    pub handle: __u8,
    pub adv_handle: __u8,
    pub num_bis: __u8,
    pub bis: hci_bis,
    pub __packed: },
pub const HCI_OP_LE_TERM_BIG: c_uint = 0x206a;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_term_big {
    pub handle: __u8,
    pub reason: __u8,
    pub __packed: },
pub const HCI_OP_LE_BIG_CREATE_SYNC: c_uint = 0x206b;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_big_create_sync {
    pub handle: __u8,
    pub sync_handle: __le16,
    pub encryption: __u8,
    pub bcode: [__u8; 16],
    pub mse: __u8,
    pub timeout: __le16,
    pub num_bis: __u8,
    pub __counted_by(num_bis): __u8 bis[],
    pub __packed: },
pub const HCI_OP_LE_BIG_TERM_SYNC: c_uint = 0x206c;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_big_term_sync {
    pub handle: __u8,
    pub __packed: },
pub const HCI_OP_LE_SETUP_ISO_PATH: c_uint = 0x206e;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_setup_iso_path {
    pub handle: __le16,
    pub direction: __u8,
    pub path: __u8,
    pub codec: __u8,
    pub codec_cid: __le16,
    pub codec_vid: __le16,
    pub delay: [__u8; 3],
    pub codec_cfg_len: __u8,
    pub codec_cfg: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_le_setup_iso_path {
    pub status: __u8,
    pub handle: __le16,
    pub __packed: },
pub const HCI_OP_LE_SET_HOST_FEATURE: c_uint = 0x2074;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_set_host_feature {
    pub bit_number: __u8,
    pub bit_value: __u8,
    pub __packed: },
pub const HCI_OP_LE_READ_ALL_LOCAL_FEATURES: c_uint = 0x2087;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_le_read_all_local_features {
    pub status: __u8,
    pub page: __u8,
    pub features: [__u8; 248],
    pub __packed: },
pub const HCI_OP_LE_READ_ALL_REMOTE_FEATURES: c_uint = 0x2088;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_read_all_remote_features {
    pub handle: __le16,
    pub pages: __u8,
    pub __packed: },
// Channel Sounding Commands
pub const HCI_OP_LE_CS_RD_LOCAL_SUPP_CAP: c_uint = 0x2089;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_le_cs_rd_local_supp_cap {
    pub status: __u8,
    pub num_config_supported: __u8,
    pub max_consecutive_procedures_supported: __le16,
    pub num_antennas_supported: __u8,
    pub max_antenna_paths_supported: __u8,
    pub roles_supported: __u8,
    pub modes_supported: __u8,
    pub rtt_capability: __u8,
    pub rtt_aa_only_n: __u8,
    pub rtt_sounding_n: __u8,
    pub rtt_random_payload_n: __u8,
    pub nadm_sounding_capability: __le16,
    pub nadm_random_capability: __le16,
    pub cs_sync_phys_supported: __u8,
    pub subfeatures_supported: __le16,
    pub t_ip1_times_supported: __le16,
    pub t_ip2_times_supported: __le16,
    pub t_fcs_times_supported: __le16,
    pub t_pm_times_supported: __le16,
    pub t_sw_time_supported: __u8,
    pub tx_snr_capability: __u8,
    pub __packed: },
pub const HCI_OP_LE_CS_RD_RMT_SUPP_CAP: c_uint = 0x208A;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_cs_rd_local_supp_cap {
    pub handle: __le16,
    pub __packed: },
pub const HCI_OP_LE_CS_WR_CACHED_RMT_SUPP_CAP: c_uint = 0x208B;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_cs_wr_cached_rmt_supp_cap {
    pub handle: __le16,
    pub num_config_supported: __u8,
    pub max_consecutive_procedures_supported: __le16,
    pub num_antennas_supported: __u8,
    pub max_antenna_paths_supported: __u8,
    pub roles_supported: __u8,
    pub modes_supported: __u8,
    pub rtt_capability: __u8,
    pub rtt_aa_only_n: __u8,
    pub rtt_sounding_n: __u8,
    pub rtt_random_payload_n: __u8,
    pub nadm_sounding_capability: __le16,
    pub nadm_random_capability: __le16,
    pub cs_sync_phys_supported: __u8,
    pub subfeatures_supported: __le16,
    pub t_ip1_times_supported: __le16,
    pub t_ip2_times_supported: __le16,
    pub t_fcs_times_supported: __le16,
    pub t_pm_times_supported: __le16,
    pub t_sw_time_supported: __u8,
    pub tx_snr_capability: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_le_cs_wr_cached_rmt_supp_cap {
    pub status: __u8,
    pub handle: __le16,
    pub __packed: },
pub const HCI_OP_LE_CS_SEC_ENABLE: c_uint = 0x208C;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_cs_sec_enable {
    pub handle: __le16,
    pub __packed: },
pub const HCI_OP_LE_CS_SET_DEFAULT_SETTINGS: c_uint = 0x208D;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_cs_set_default_settings {
    pub handle: __le16,
    pub role_enable: __u8,
    pub cs_sync_ant_sel: __u8,
    pub max_tx_power: __s8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_le_cs_set_default_settings {
    pub status: __u8,
    pub handle: __le16,
    pub __packed: },
pub const HCI_OP_LE_CS_RD_RMT_FAE_TABLE: c_uint = 0x208E;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_cs_rd_rmt_fae_table {
    pub handle: __le16,
    pub __packed: },
pub const HCI_OP_LE_CS_WR_CACHED_RMT_FAE_TABLE: c_uint = 0x208F;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_cs_wr_rmt_cached_fae_table {
    pub handle: __le16,
    pub remote_fae_table: [__u8; 72],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_le_cs_wr_rmt_cached_fae_table {
    pub status: __u8,
    pub handle: __le16,
    pub __packed: },
pub const HCI_OP_LE_CS_CREATE_CONFIG: c_uint = 0x2090;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_cs_create_config {
    pub handle: __le16,
    pub config_id: __u8,
    pub create_context: __u8,
    pub main_mode_type: __u8,
    pub sub_mode_type: __u8,
    pub min_main_mode_steps: __u8,
    pub max_main_mode_steps: __u8,
    pub main_mode_repetition: __u8,
    pub mode_0_steps: __u8,
    pub role: __u8,
    pub rtt_type: __u8,
    pub cs_sync_phy: __u8,
    pub channel_map: [__u8; 10],
    pub channel_map_repetition: __u8,
    pub channel_selection_type: __u8,
    pub ch3c_shape: __u8,
    pub ch3c_jump: __u8,
    pub reserved: __u8,
    pub __packed: },
pub const HCI_OP_LE_CS_REMOVE_CONFIG: c_uint = 0x2091;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_cs_remove_config {
    pub handle: __le16,
    pub config_id: __u8,
    pub __packed: },
pub const HCI_OP_LE_CS_SET_CH_CLASSIFICATION: c_uint = 0x2092;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_cs_set_ch_classification {
    pub ch_classification: [__u8; 10],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_le_cs_set_ch_classification {
    pub status: __u8,
    pub __packed: },
pub const HCI_OP_LE_CS_SET_PROC_PARAM: c_uint = 0x2093;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_cs_set_proc_param {
    pub handle: __le16,
    pub config_id: __u8,
    pub max_procedure_len: __le16,
    pub min_procedure_interval: __le16,
    pub max_procedure_interval: __le16,
    pub max_procedure_count: __le16,
    pub min_subevent_len: [__u8; 3],
    pub max_subevent_len: [__u8; 3],
    pub tone_antenna_config_selection: __u8,
    pub phy: __u8,
    pub tx_power_delta: __u8,
    pub preferred_peer_antenna: __u8,
    pub snr_control_initiator: __u8,
    pub snr_control_reflector: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_le_cs_set_proc_param {
    pub status: __u8,
    pub handle: __le16,
    pub __packed: },
pub const HCI_OP_LE_CS_SET_PROC_ENABLE: c_uint = 0x2094;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_cs_set_proc_enable {
    pub handle: __le16,
    pub config_id: __u8,
    pub enable: __u8,
    pub __packed: },
pub const HCI_OP_LE_CS_TEST: c_uint = 0x2095;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_cs_test {
    pub main_mode_type: __u8,
    pub sub_mode_type: __u8,
    pub main_mode_repetition: __u8,
    pub mode_0_steps: __u8,
    pub role: __u8,
    pub rtt_type: __u8,
    pub cs_sync_phy: __u8,
    pub cs_sync_antenna_selection: __u8,
    pub subevent_len: [__u8; 3],
    pub subevent_interval: __le16,
    pub max_num_subevents: __u8,
    pub transmit_power_level: __u8,
    pub t_ip1_time: __u8,
    pub t_ip2_time: __u8,
    pub t_fcs_time: __u8,
    pub t_pm_time: __u8,
    pub t_sw_time: __u8,
    pub tone_antenna_config_selection: __u8,
    pub reserved: __u8,
    pub snr_control_initiator: __u8,
    pub snr_control_reflector: __u8,
    pub drbg_nonce: __le16,
    pub channel_map_repetition: __u8,
    pub override_config: __le16,
    pub override_parameters_length: __u8,
    pub override_parameters_data: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_le_cs_test {
    pub status: __u8,
    pub __packed: },
pub const HCI_OP_LE_CS_TEST_END: c_uint = 0x2096;
pub const HCI_OP_LE_SET_HOST_FEATURE_V2: c_uint = 0x2097;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_set_host_feature_v2 {
    pub bit_number: __le16,
    pub bit_value: __u8,
    pub __packed: },
pub const HCI_OP_LE_CONN_RATE: c_uint = 0x20a1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_conn_rate {
    pub handle: __le16,
    pub interval_min: __le16,
    pub interval_max: __le16,
    pub subrate_min: __le16,
    pub subrate_max: __le16,
    pub max_latency: __le16,
    pub cont_num: __le16,
    pub supv_timeout: __le16,
    pub min_ce_len: __le16,
    pub max_ce_len: __le16,
    pub __packed: },
pub const HCI_OP_LE_SET_DEF_RATE: c_uint = 0x20a2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cp_le_set_def_rate {
    pub interval_min: __le16,
    pub interval_max: __le16,
    pub subrate_min: __le16,
    pub subrate_max: __le16,
    pub max_latency: __le16,
    pub cont_num: __le16,
    pub supv_timeout: __le16,
    pub min_ce_len: __le16,
    pub max_ce_len: __le16,
    pub __packed: },
pub const HCI_OP_LE_READ_CONN_INTERVAL: c_uint = 0x20a3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_le_conn_interval_group {
    pub min: __le16,
    pub max: __le16,
    pub stride: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_rp_le_read_conn_interval {
    pub status: __u8,
    pub num_grps: __u8,
    pub grps: [hci_le_conn_interval_group; ],
    pub __packed: },
// ---- HCI Events ----
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_status {
    pub status: __u8,
    pub __packed: },
pub const HCI_EV_INQUIRY_COMPLETE: c_uint = 0x01;
pub const HCI_EV_INQUIRY_RESULT: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inquiry_info {
    pub bdaddr: bdaddr_t,
    pub pscan_rep_mode: __u8,
    pub pscan_period_mode: __u8,
    pub pscan_mode: __u8,
    pub dev_class: [__u8; 3],
    pub clock_offset: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_inquiry_result {
    pub num: __u8,
    pub info: [inquiry_info; ],
}

pub const HCI_EV_CONN_COMPLETE: c_uint = 0x03;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_conn_complete {
    pub status: __u8,
    pub handle: __le16,
    pub bdaddr: bdaddr_t,
    pub link_type: __u8,
    pub encr_mode: __u8,
    pub __packed: },
pub const HCI_EV_CONN_REQUEST: c_uint = 0x04;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_conn_request {
    pub bdaddr: bdaddr_t,
    pub dev_class: [__u8; 3],
    pub link_type: __u8,
    pub __packed: },
pub const HCI_EV_DISCONN_COMPLETE: c_uint = 0x05;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_disconn_complete {
    pub status: __u8,
    pub handle: __le16,
    pub reason: __u8,
    pub __packed: },
pub const HCI_EV_AUTH_COMPLETE: c_uint = 0x06;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_auth_complete {
    pub status: __u8,
    pub handle: __le16,
    pub __packed: },
pub const HCI_EV_REMOTE_NAME: c_uint = 0x07;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_remote_name {
    pub status: __u8,
    pub bdaddr: bdaddr_t,
    pub name: [__u8; HCI_MAX_NAME_LENGTH],
    pub __packed: },
pub const HCI_EV_ENCRYPT_CHANGE: c_uint = 0x08;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_encrypt_change {
    pub status: __u8,
    pub handle: __le16,
    pub encrypt: __u8,
    pub __packed: },
pub const HCI_EV_CHANGE_LINK_KEY_COMPLETE: c_uint = 0x09;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_change_link_key_complete {
    pub status: __u8,
    pub handle: __le16,
    pub __packed: },
pub const HCI_EV_REMOTE_FEATURES: c_uint = 0x0b;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_remote_features {
    pub status: __u8,
    pub handle: __le16,
    pub features: [__u8; 8],
    pub __packed: },
pub const HCI_EV_REMOTE_VERSION: c_uint = 0x0c;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_remote_version {
    pub status: __u8,
    pub handle: __le16,
    pub lmp_ver: __u8,
    pub manufacturer: __le16,
    pub lmp_subver: __le16,
    pub __packed: },
pub const HCI_EV_QOS_SETUP_COMPLETE: c_uint = 0x0d;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_qos {
    pub service_type: __u8,
    pub token_rate: __u32,
    pub peak_bandwidth: __u32,
    pub latency: __u32,
    pub delay_variation: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_qos_setup_complete {
    pub status: __u8,
    pub handle: __le16,
    pub qos: hci_qos,
    pub __packed: },
pub const HCI_EV_CMD_COMPLETE: c_uint = 0x0e;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_cmd_complete {
    pub ncmd: __u8,
    pub opcode: __le16,
    pub __packed: },
pub const HCI_EV_CMD_STATUS: c_uint = 0x0f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_cmd_status {
    pub status: __u8,
    pub ncmd: __u8,
    pub opcode: __le16,
    pub __packed: },
pub const HCI_EV_HARDWARE_ERROR: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_hardware_error {
    pub code: __u8,
    pub __packed: },
pub const HCI_EV_ROLE_CHANGE: c_uint = 0x12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_role_change {
    pub status: __u8,
    pub bdaddr: bdaddr_t,
    pub role: __u8,
    pub __packed: },
pub const HCI_EV_NUM_COMP_PKTS: c_uint = 0x13;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_comp_pkts_info {
    pub handle: __le16,
    pub count: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_num_comp_pkts {
    pub num: __u8,
    pub handles: [hci_comp_pkts_info; ],
    pub __packed: },
pub const HCI_EV_MODE_CHANGE: c_uint = 0x14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_mode_change {
    pub status: __u8,
    pub handle: __le16,
    pub mode: __u8,
    pub interval: __le16,
    pub __packed: },
pub const HCI_EV_PIN_CODE_REQ: c_uint = 0x16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_pin_code_req {
    pub bdaddr: bdaddr_t,
    pub __packed: },
pub const HCI_EV_LINK_KEY_REQ: c_uint = 0x17;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_link_key_req {
    pub bdaddr: bdaddr_t,
    pub __packed: },
pub const HCI_EV_LINK_KEY_NOTIFY: c_uint = 0x18;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_link_key_notify {
    pub bdaddr: bdaddr_t,
    pub link_key: [__u8; HCI_LINK_KEY_SIZE],
    pub key_type: __u8,
    pub __packed: },
pub const HCI_EV_CLOCK_OFFSET: c_uint = 0x1c;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_clock_offset {
    pub status: __u8,
    pub handle: __le16,
    pub clock_offset: __le16,
    pub __packed: },
pub const HCI_EV_PKT_TYPE_CHANGE: c_uint = 0x1d;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_pkt_type_change {
    pub status: __u8,
    pub handle: __le16,
    pub pkt_type: __le16,
    pub __packed: },
pub const HCI_EV_PSCAN_REP_MODE: c_uint = 0x20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_pscan_rep_mode {
    pub bdaddr: bdaddr_t,
    pub pscan_rep_mode: __u8,
    pub __packed: },
pub const HCI_EV_INQUIRY_RESULT_WITH_RSSI: c_uint = 0x22;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inquiry_info_rssi {
    pub bdaddr: bdaddr_t,
    pub pscan_rep_mode: __u8,
    pub pscan_period_mode: __u8,
    pub dev_class: [__u8; 3],
    pub clock_offset: __le16,
    pub rssi: __s8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inquiry_info_rssi_pscan {
    pub bdaddr: bdaddr_t,
    pub pscan_rep_mode: __u8,
    pub pscan_period_mode: __u8,
    pub pscan_mode: __u8,
    pub dev_class: [__u8; 3],
    pub clock_offset: __le16,
    pub rssi: __s8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_inquiry_result_rssi {
    pub num: __u8,
    pub data: [__u8; ],
    pub __packed: },
pub const HCI_EV_REMOTE_EXT_FEATURES: c_uint = 0x23;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_remote_ext_features {
    pub status: __u8,
    pub handle: __le16,
    pub page: __u8,
    pub max_page: __u8,
    pub features: [__u8; 8],
    pub __packed: },
pub const HCI_EV_SYNC_CONN_COMPLETE: c_uint = 0x2c;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_sync_conn_complete {
    pub status: __u8,
    pub handle: __le16,
    pub bdaddr: bdaddr_t,
    pub link_type: __u8,
    pub tx_interval: __u8,
    pub retrans_window: __u8,
    pub rx_pkt_len: __le16,
    pub tx_pkt_len: __le16,
    pub air_mode: __u8,
    pub __packed: },
pub const HCI_EV_SYNC_CONN_CHANGED: c_uint = 0x2d;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_sync_conn_changed {
    pub status: __u8,
    pub handle: __le16,
    pub tx_interval: __u8,
    pub retrans_window: __u8,
    pub rx_pkt_len: __le16,
    pub tx_pkt_len: __le16,
    pub __packed: },
pub const HCI_EV_SNIFF_SUBRATE: c_uint = 0x2e;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_sniff_subrate {
    pub status: __u8,
    pub handle: __le16,
    pub max_tx_latency: __le16,
    pub max_rx_latency: __le16,
    pub max_remote_timeout: __le16,
    pub max_local_timeout: __le16,
    pub __packed: },
pub const HCI_EV_EXTENDED_INQUIRY_RESULT: c_uint = 0x2f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct extended_inquiry_info {
    pub bdaddr: bdaddr_t,
    pub pscan_rep_mode: __u8,
    pub pscan_period_mode: __u8,
    pub dev_class: [__u8; 3],
    pub clock_offset: __le16,
    pub rssi: __s8,
    pub data: [__u8; 240],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_ext_inquiry_result {
    pub num: __u8,
    pub info: [extended_inquiry_info; ],
    pub __packed: },
pub const HCI_EV_KEY_REFRESH_COMPLETE: c_uint = 0x30;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_key_refresh_complete {
    pub status: __u8,
    pub handle: __le16,
    pub __packed: },
pub const HCI_EV_IO_CAPA_REQUEST: c_uint = 0x31;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_io_capa_request {
    pub bdaddr: bdaddr_t,
    pub __packed: },
pub const HCI_EV_IO_CAPA_REPLY: c_uint = 0x32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_io_capa_reply {
    pub bdaddr: bdaddr_t,
    pub capability: __u8,
    pub oob_data: __u8,
    pub authentication: __u8,
    pub __packed: },
pub const HCI_EV_USER_CONFIRM_REQUEST: c_uint = 0x33;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_user_confirm_req {
    pub bdaddr: bdaddr_t,
    pub passkey: __le32,
    pub __packed: },
pub const HCI_EV_USER_PASSKEY_REQUEST: c_uint = 0x34;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_user_passkey_req {
    pub bdaddr: bdaddr_t,
    pub __packed: },
pub const HCI_EV_REMOTE_OOB_DATA_REQUEST: c_uint = 0x35;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_remote_oob_data_request {
    pub bdaddr: bdaddr_t,
    pub __packed: },
pub const HCI_EV_SIMPLE_PAIR_COMPLETE: c_uint = 0x36;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_simple_pair_complete {
    pub status: __u8,
    pub bdaddr: bdaddr_t,
    pub __packed: },
pub const HCI_EV_USER_PASSKEY_NOTIFY: c_uint = 0x3b;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_user_passkey_notify {
    pub bdaddr: bdaddr_t,
    pub passkey: __le32,
    pub __packed: },
pub const HCI_KEYPRESS_STARTED: c_int = 0;
pub const HCI_KEYPRESS_ENTERED: c_int = 1;
pub const HCI_KEYPRESS_ERASED: c_int = 2;
pub const HCI_KEYPRESS_CLEARED: c_int = 3;
pub const HCI_KEYPRESS_COMPLETED: c_int = 4;
pub const HCI_EV_KEYPRESS_NOTIFY: c_uint = 0x3c;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_keypress_notify {
    pub bdaddr: bdaddr_t,
    pub type: __u8,
    pub __packed: },
pub const HCI_EV_REMOTE_HOST_FEATURES: c_uint = 0x3d;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_remote_host_features {
    pub bdaddr: bdaddr_t,
    pub features: [__u8; 8],
    pub __packed: },
pub const HCI_EV_LE_META: c_uint = 0x3e;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_le_meta {
    pub subevent: __u8,
    pub __packed: },
pub const HCI_EV_PHY_LINK_COMPLETE: c_uint = 0x40;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_phy_link_complete {
    pub status: __u8,
    pub phy_handle: __u8,
    pub __packed: },
pub const HCI_EV_CHANNEL_SELECTED: c_uint = 0x41;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_channel_selected {
    pub phy_handle: __u8,
    pub __packed: },
pub const HCI_EV_DISCONN_PHY_LINK_COMPLETE: c_uint = 0x42;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_disconn_phy_link_complete {
    pub status: __u8,
    pub phy_handle: __u8,
    pub reason: __u8,
    pub __packed: },
pub const HCI_EV_LOGICAL_LINK_COMPLETE: c_uint = 0x45;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_logical_link_complete {
    pub status: __u8,
    pub handle: __le16,
    pub phy_handle: __u8,
    pub flow_spec_id: __u8,
    pub __packed: },
pub const HCI_EV_DISCONN_LOGICAL_LINK_COMPLETE: c_uint = 0x46;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_disconn_logical_link_complete {
    pub status: __u8,
    pub handle: __le16,
    pub reason: __u8,
    pub __packed: },
pub const HCI_EV_NUM_COMP_BLOCKS: c_uint = 0x48;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_comp_blocks_info {
    pub handle: __le16,
    pub pkts: __le16,
    pub blocks: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_num_comp_blocks {
    pub num_blocks: __le16,
    pub num_hndl: __u8,
    pub handles: [hci_comp_blocks_info; ],
    pub __packed: },
pub const HCI_EV_SYNC_TRAIN_COMPLETE: c_uint = 0x4F;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_sync_train_complete {
    pub status: __u8,
    pub __packed: },
pub const HCI_EV_PERIPHERAL_PAGE_RESP_TIMEOUT: c_uint = 0x54;
pub const HCI_EV_LE_CONN_COMPLETE: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_le_conn_complete {
    pub status: __u8,
    pub handle: __le16,
    pub role: __u8,
    pub bdaddr_type: __u8,
    pub bdaddr: bdaddr_t,
    pub interval: __le16,
    pub latency: __le16,
    pub supervision_timeout: __le16,
    pub clk_accurancy: __u8,
    pub __packed: },
// Advertising report event types
pub const LE_ADV_IND: c_uint = 0x00;
pub const LE_ADV_DIRECT_IND: c_uint = 0x01;
pub const LE_ADV_SCAN_IND: c_uint = 0x02;
pub const LE_ADV_NONCONN_IND: c_uint = 0x03;
pub const LE_ADV_SCAN_RSP: c_uint = 0x04;
pub const LE_ADV_INVALID: c_uint = 0x05;
// Legacy event types in extended adv report
pub const LE_LEGACY_ADV_IND: c_uint = 0x0013;
pub const LE_LEGACY_ADV_DIRECT_IND: c_uint = 0x0015;
pub const LE_LEGACY_ADV_SCAN_IND: c_uint = 0x0012;
pub const LE_LEGACY_NONCONN_IND: c_uint = 0x0010;
pub const LE_LEGACY_SCAN_RSP_ADV: c_uint = 0x001b;
pub const LE_LEGACY_SCAN_RSP_ADV_SCAN: c_uint = 0x001a;
// Extended Advertising event types
pub const LE_EXT_ADV_NON_CONN_IND: c_uint = 0x0000;
pub const LE_EXT_ADV_CONN_IND: c_uint = 0x0001;
pub const LE_EXT_ADV_SCAN_IND: c_uint = 0x0002;
pub const LE_EXT_ADV_DIRECT_IND: c_uint = 0x0004;
pub const LE_EXT_ADV_SCAN_RSP: c_uint = 0x0008;
pub const LE_EXT_ADV_LEGACY_PDU: c_uint = 0x0010;
pub const LE_EXT_ADV_DATA_STATUS_MASK: c_uint = 0x0060;
pub const LE_EXT_ADV_EVT_TYPE_MASK: c_uint = 0x007f;
pub const ADDR_LE_DEV_PUBLIC: c_uint = 0x00;
pub const ADDR_LE_DEV_RANDOM: c_uint = 0x01;
pub const ADDR_LE_DEV_PUBLIC_RESOLVED: c_uint = 0x02;
pub const ADDR_LE_DEV_RANDOM_RESOLVED: c_uint = 0x03;
pub const HCI_EV_LE_ADVERTISING_REPORT: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_le_advertising_info {
    pub type: __u8,
    pub bdaddr_type: __u8,
    pub bdaddr: bdaddr_t,
    pub length: __u8,
    pub data: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_le_advertising_report {
    pub num: __u8,
    pub info: [hci_ev_le_advertising_info; ],
    pub __packed: },
pub const HCI_EV_LE_CONN_UPDATE_COMPLETE: c_uint = 0x03;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_le_conn_update_complete {
    pub status: __u8,
    pub handle: __le16,
    pub interval: __le16,
    pub latency: __le16,
    pub supervision_timeout: __le16,
    pub __packed: },
pub const HCI_EV_LE_REMOTE_FEAT_COMPLETE: c_uint = 0x04;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_le_remote_feat_complete {
    pub status: __u8,
    pub handle: __le16,
    pub features: [__u8; 8],
    pub __packed: },
pub const HCI_EV_LE_LTK_REQ: c_uint = 0x05;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_le_ltk_req {
    pub handle: __le16,
    pub rand: __le64,
    pub ediv: __le16,
    pub __packed: },
pub const HCI_EV_LE_REMOTE_CONN_PARAM_REQ: c_uint = 0x06;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_le_remote_conn_param_req {
    pub handle: __le16,
    pub interval_min: __le16,
    pub interval_max: __le16,
    pub latency: __le16,
    pub timeout: __le16,
    pub __packed: },
pub const HCI_EV_LE_DATA_LEN_CHANGE: c_uint = 0x07;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_le_data_len_change {
    pub handle: __le16,
    pub tx_len: __le16,
    pub tx_time: __le16,
    pub rx_len: __le16,
    pub rx_time: __le16,
    pub __packed: },
pub const HCI_EV_LE_DIRECT_ADV_REPORT: c_uint = 0x0B;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_le_direct_adv_info {
    pub type: __u8,
    pub bdaddr_type: __u8,
    pub bdaddr: bdaddr_t,
    pub direct_addr_type: __u8,
    pub direct_addr: bdaddr_t,
    pub rssi: __s8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_le_direct_adv_report {
    pub num: __u8,
    pub info: [hci_ev_le_direct_adv_info; ],
    pub __packed: },
pub const HCI_EV_LE_PHY_UPDATE_COMPLETE: c_uint = 0x0c;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_le_phy_update_complete {
    pub status: __u8,
    pub handle: __le16,
    pub tx_phy: __u8,
    pub rx_phy: __u8,
    pub __packed: },
pub const HCI_EV_LE_EXT_ADV_REPORT: c_uint = 0x0d;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_le_ext_adv_info {
    pub type: __le16,
    pub bdaddr_type: __u8,
    pub bdaddr: bdaddr_t,
    pub primary_phy: __u8,
    pub secondary_phy: __u8,
    pub sid: __u8,
    pub tx_power: __u8,
    pub rssi: __s8,
    pub interval: __le16,
    pub direct_addr_type: __u8,
    pub direct_addr: bdaddr_t,
    pub length: __u8,
    pub data: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_le_ext_adv_report {
    pub num: __u8,
    pub info: [hci_ev_le_ext_adv_info; ],
    pub __packed: },
pub const HCI_EV_LE_PA_SYNC_ESTABLISHED: c_uint = 0x0e;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_le_pa_sync_established {
    pub status: __u8,
    pub handle: __le16,
    pub sid: __u8,
    pub bdaddr_type: __u8,
    pub bdaddr: bdaddr_t,
    pub phy: __u8,
    pub interval: __le16,
    pub clock_accuracy: __u8,
    pub __packed: },
pub const HCI_EV_LE_ENHANCED_CONN_COMPLETE: c_uint = 0x0a;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_le_enh_conn_complete {
    pub status: __u8,
    pub handle: __le16,
    pub role: __u8,
    pub bdaddr_type: __u8,
    pub bdaddr: bdaddr_t,
    pub local_rpa: bdaddr_t,
    pub peer_rpa: bdaddr_t,
    pub interval: __le16,
    pub latency: __le16,
    pub supervision_timeout: __le16,
    pub clk_accurancy: __u8,
    pub __packed: },
pub const HCI_EV_LE_PER_ADV_REPORT: c_uint = 0x0f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_le_per_adv_report {
    pub sync_handle: __le16,
    pub tx_power: __u8,
    pub rssi: __u8,
    pub cte_type: __u8,
    pub data_status: __u8,
    pub length: __u8,
    pub data: [__u8; ],
    pub __packed: },
pub const HCI_EV_LE_PA_SYNC_LOST: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_le_pa_sync_lost {
    pub handle: __le16,
    pub __packed: },
pub const LE_PA_DATA_COMPLETE: c_uint = 0x00;
pub const LE_PA_DATA_MORE_TO_COME: c_uint = 0x01;
pub const LE_PA_DATA_TRUNCATED: c_uint = 0x02;
pub const HCI_EV_LE_EXT_ADV_SET_TERM: c_uint = 0x12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_evt_le_ext_adv_set_term {
    pub status: __u8,
    pub handle: __u8,
    pub conn_handle: __le16,
    pub num_evts: __u8,
    pub __packed: },
pub const HCI_EV_LE_PAST_RECEIVED: c_uint = 0x18;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_le_past_received {
    pub status: __u8,
    pub handle: __le16,
    pub service_data: __le16,
    pub sync_handle: __le16,
    pub sid: __u8,
    pub bdaddr_type: __u8,
    pub bdaddr: bdaddr_t,
    pub phy: __u8,
    pub interval: __le16,
    pub clock_accuracy: __u8,
    pub __packed: },
pub const HCI_EVT_LE_CIS_ESTABLISHED: c_uint = 0x19;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_evt_le_cis_established {
    pub status: __u8,
    pub handle: __le16,
    pub cig_sync_delay: [__u8; 3],
    pub cis_sync_delay: [__u8; 3],
    pub c_latency: [__u8; 3],
    pub p_latency: [__u8; 3],
    pub c_phy: __u8,
    pub p_phy: __u8,
    pub nse: __u8,
    pub c_bn: __u8,
    pub p_bn: __u8,
    pub c_ft: __u8,
    pub p_ft: __u8,
    pub c_mtu: __le16,
    pub p_mtu: __le16,
    pub interval: __le16,
    pub __packed: },
pub const HCI_EVT_LE_CIS_REQ: c_uint = 0x1a;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_evt_le_cis_req {
    pub acl_handle: __le16,
    pub cis_handle: __le16,
    pub cig_id: __u8,
    pub cis_id: __u8,
    pub __packed: },
pub const HCI_EVT_LE_CREATE_BIG_COMPLETE: c_uint = 0x1b;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_evt_le_create_big_complete {
    pub status: __u8,
    pub handle: __u8,
    pub sync_delay: [__u8; 3],
    pub transport_delay: [__u8; 3],
    pub phy: __u8,
    pub nse: __u8,
    pub bn: __u8,
    pub pto: __u8,
    pub irc: __u8,
    pub max_pdu: __le16,
    pub interval: __le16,
    pub num_bis: __u8,
    pub bis_handle: [__le16; ],
    pub __packed: },
pub const HCI_EVT_LE_BIG_SYNC_ESTABLISHED: c_uint = 0x1d;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_evt_le_big_sync_established {
    pub status: __u8,
    pub handle: __u8,
    pub latency: [__u8; 3],
    pub nse: __u8,
    pub bn: __u8,
    pub pto: __u8,
    pub irc: __u8,
    pub max_pdu: __le16,
    pub interval: __le16,
    pub num_bis: __u8,
    pub bis: [__le16; ],
    pub __packed: },
pub const HCI_EVT_LE_BIG_SYNC_LOST: c_uint = 0x1e;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_evt_le_big_sync_lost {
    pub handle: __u8,
    pub reason: __u8,
    pub __packed: },
pub const HCI_EVT_LE_BIG_INFO_ADV_REPORT: c_uint = 0x22;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_evt_le_big_info_adv_report {
    pub sync_handle: __le16,
    pub num_bis: __u8,
    pub nse: __u8,
    pub iso_interval: __le16,
    pub bn: __u8,
    pub pto: __u8,
    pub irc: __u8,
    pub max_pdu: __le16,
    pub sdu_interval: [__u8; 3],
    pub max_sdu: __le16,
    pub phy: __u8,
    pub framing: __u8,
    pub encryption: __u8,
    pub __packed: },
pub const HCI_EVT_LE_ALL_REMOTE_FEATURES_COMPLETE: c_uint = 0x2b;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_evt_le_read_all_remote_features_complete {
    pub status: __u8,
    pub handle: __le16,
    pub max_pages: __u8,
    pub valid_pages: __u8,
    pub features: [__u8; 248],
    pub __packed: },
// Channel Sounding Events
pub const HCI_EVT_LE_CS_READ_RMT_SUPP_CAP_COMPLETE: c_uint = 0x2C;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_evt_le_cs_read_rmt_supp_cap_complete {
    pub status: __u8,
    pub handle: __le16,
    pub num_configs_supp: __u8,
    pub max_consec_proc_supp: __le16,
    pub num_ant_supp: __u8,
    pub max_ant_path_supp: __u8,
    pub roles_supp: __u8,
    pub modes_supp: __u8,
    pub rtt_cap: __u8,
    pub rtt_aa_only_n: __u8,
    pub rtt_sounding_n: __u8,
    pub rtt_rand_payload_n: __u8,
    pub nadm_sounding_cap: __le16,
    pub nadm_rand_cap: __le16,
    pub cs_sync_phys_supp: __u8,
    pub sub_feat_supp: __le16,
    pub t_ip1_times_supp: __le16,
    pub t_ip2_times_supp: __le16,
    pub t_fcs_times_supp: __le16,
    pub t_pm_times_supp: __le16,
    pub t_sw_times_supp: __u8,
    pub tx_snr_cap: __u8,
    pub __packed: },
pub const HCI_EVT_LE_CS_READ_RMT_FAE_TABLE_COMPLETE: c_uint = 0x2D;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_evt_le_cs_read_rmt_fae_table_complete {
    pub status: __u8,
    pub handle: __le16,
    pub remote_fae_table: [__u8; 72],
    pub __packed: },
pub const HCI_EVT_LE_CS_SECURITY_ENABLE_COMPLETE: c_uint = 0x2E;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_evt_le_cs_security_enable_complete {
    pub status: __u8,
    pub handle: __le16,
    pub __packed: },
pub const HCI_EVT_LE_CS_CONFIG_COMPLETE: c_uint = 0x2F;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_evt_le_cs_config_complete {
    pub status: __u8,
    pub handle: __le16,
    pub config_id: __u8,
    pub action: __u8,
    pub main_mode_type: __u8,
    pub sub_mode_type: __u8,
    pub min_main_mode_steps: __u8,
    pub max_main_mode_steps: __u8,
    pub main_mode_rep: __u8,
    pub mode_0_steps: __u8,
    pub role: __u8,
    pub rtt_type: __u8,
    pub cs_sync_phy: __u8,
    pub channel_map: [__u8; 10],
    pub channel_map_rep: __u8,
    pub channel_sel_type: __u8,
    pub ch3c_shape: __u8,
    pub ch3c_jump: __u8,
    pub reserved: __u8,
    pub t_ip1_time: __u8,
    pub t_ip2_time: __u8,
    pub t_fcs_time: __u8,
    pub t_pm_time: __u8,
    pub __packed: },
pub const HCI_EVT_LE_CS_PROCEDURE_ENABLE_COMPLETE: c_uint = 0x30;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_evt_le_cs_procedure_enable_complete {
    pub status: __u8,
    pub handle: __le16,
    pub config_id: __u8,
    pub state: __u8,
    pub tone_ant_config_sel: __u8,
    pub sel_tx_pwr: __s8,
    pub sub_evt_len: [__u8; 3],
    pub sub_evts_per_evt: __u8,
    pub sub_evt_intrvl: __le16,
    pub evt_intrvl: __le16,
    pub proc_intrvl: __le16,
    pub proc_counter: __le16,
    pub max_proc_len: __le16,
    pub __packed: },
pub const HCI_EVT_LE_CS_SUBEVENT_RESULT: c_uint = 0x31;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_evt_le_cs_subevent_result {
    pub handle: __le16,
    pub config_id: __u8,
    pub start_acl_conn_evt_counter: __le16,
    pub proc_counter: __le16,
    pub freq_comp: __le16,
    pub ref_pwr_lvl: __u8,
    pub proc_done_status: __u8,
    pub subevt_done_status: __u8,
    pub abort_reason: __u8,
    pub num_ant_paths: __u8,
    pub num_steps_reported: __u8,
    pub /: *mut *mut __u8 step_mode[0]; / depends on num_steps_reported,
    pub /: *mut *mut __u8 step_channel[0]; / depends on num_steps_reported,
    pub /: *mut *mut __u8 step_data_length[0]; / depends on num_steps_reported,
    pub /: *mut *mut __u8 step_data[0]; / depends on num_steps_reported,
    pub __packed: },
pub const HCI_EVT_LE_CS_SUBEVENT_RESULT_CONTINUE: c_uint = 0x32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_evt_le_cs_subevent_result_continue {
    pub handle: __le16,
    pub config_id: __u8,
    pub proc_done_status: __u8,
    pub subevt_done_status: __u8,
    pub abort_reason: __u8,
    pub num_ant_paths: __u8,
    pub num_steps_reported: __u8,
    pub /: *mut *mut __u8 step_mode[0]; / depends on num_steps_reported,
    pub /: *mut *mut __u8 step_channel[0]; / depends on num_steps_reported,
    pub /: *mut *mut __u8 step_data_length[0]; / depends on num_steps_reported,
    pub /: *mut *mut __u8 step_data[0]; / depends on num_steps_reported,
    pub __packed: },
pub const HCI_EVT_LE_CS_TEST_END_COMPLETE: c_uint = 0x33;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_evt_le_cs_test_end_complete {
    pub status: __u8,
    pub __packed: },
pub const HCI_EVT_LE_CONN_RATE_CHANGE: c_uint = 0x37;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_evt_le_conn_rate_change {
    pub status: __u8,
    pub handle: __le16,
    pub interval: __le16,
    pub subrate: __le16,
    pub latency: __le16,
    pub cont_number: __le16,
    pub supv_timeout: __le16,
    pub __packed: },
pub const HCI_EV_VENDOR: c_uint = 0xff;
// Internal events generated by Bluetooth stack
pub const HCI_EV_STACK_INTERNAL: c_uint = 0xfd;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_stack_internal {
    pub type: __u16,
    pub data: [__u8; ],
    pub __packed: },
pub const HCI_EV_SI_DEVICE: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_si_device {
    pub event: __u16,
    pub dev_id: __u16,
    pub __packed: },
pub const HCI_EV_SI_SECURITY: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ev_si_security {
    pub event: __u16,
    pub proto: __u16,
    pub subproto: __u16,
    pub incoming: __u8,
    pub __packed: },
// ---- HCI Packet structures ----
pub const HCI_COMMAND_HDR_SIZE: c_int = 3;
pub const HCI_EVENT_HDR_SIZE: c_int = 2;
pub const HCI_MAX_EVENT_PLEN: c_int = 255;
pub const HCI_ACL_HDR_SIZE: c_int = 4;
pub const HCI_SCO_HDR_SIZE: c_int = 3;
pub const HCI_ISO_HDR_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_command_hdr {
    pub /: *mut *mut __le16 opcode; / OCF & OGF,
    pub plen: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_event_hdr {
    pub evt: __u8,
    pub plen: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_acl_hdr {
    pub /: *mut *mut __le16 handle; / Handle & Flags(PB, BC),
    pub dlen: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_sco_hdr {
    pub handle: __le16,
    pub dlen: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_iso_hdr {
    pub handle: __le16,
    pub dlen: __le16,
    pub data: [__u8; ],
    pub __packed: },
// ISO data packet status flags
pub const HCI_ISO_STATUS_VALID: c_uint = 0x00;
pub const HCI_ISO_STATUS_INVALID: c_uint = 0x01;
pub const HCI_ISO_STATUS_NOP: c_uint = 0x02;
pub const HCI_ISO_DATA_HDR_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_iso_data_hdr {
    pub sn: __le16,
    pub slen: __le16,
}

pub const HCI_ISO_TS_DATA_HDR_SIZE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_iso_ts_data_hdr {
    pub ts: __le32,
    pub sn: __le16,
    pub slen: __le16,
}

// Command opcode pack/unpack

// ACL handle and flags pack/unpack

extern "C" {
    pub fn hci_handle(_arg: __le16_to_cpu(hci_acl_hdr(skb)->handle)) -> return;
}
extern "C" {
    pub fn __le16_to_cpu(_arg: hci_acl_hdr(skb)->dlen) -> return;
}
// ISO handle and flags pack/unpack

// ISO data length and flags pack/unpack

// codec transport types
pub const HCI_TRANSPORT_SCO_ESCO: c_uint = 0x01;
// le24 support
