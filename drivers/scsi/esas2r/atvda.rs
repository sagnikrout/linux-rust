//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/esas2r/atvda.h
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


// linux/drivers/scsi/esas2r/atvda.h
// ATTO VDA interface definitions
//
// Copyright (c) 2001-2013 ATTO Technology, Inc.
// (mailto:linuxdrivers@attotech.com)
//
// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; version 2 of the License.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// NO WARRANTY
// THE PROGRAM IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, EITHER EXPRESS OR IMPLIED INCLUDING, WITHOUT
// LIMITATION, ANY WARRANTIES OR CONDITIONS OF TITLE, NON-INFRINGEMENT,
// MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE. Each Recipient is
// solely responsible for determining the appropriateness of using and
// distributing the Program and assumes all risks associated with its
// exercise of rights under this Agreement, including but not limited to
// the risks and costs of program errors, damage to or loss of data,
// programs or equipment, and unavailability or interruption of operations.
//
// DISCLAIMER OF LIABILITY
// NEITHER RECIPIENT NOR ANY CONTRIBUTORS SHALL HAVE ANY LIABILITY FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING WITHOUT LIMITATION LOST PROFITS), HOWEVER CAUSED AND
// ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR
// TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OR DISTRIBUTION OF THE PROGRAM OR THE EXERCISE OF ANY RIGHTS GRANTED
// HEREUNDER, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGES
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
//
// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
pub const VDA_DEVADDRF_SATA: c_uint = 0x01;
pub const VDA_DEVADDRF_SSD: c_uint = 0x02;
// dev_addr2 was added for 64-bit alignment
// VDA request function codes
pub const VDA_FUNC_SCSI: c_uint = 0x00;
pub const VDA_FUNC_FLASH: c_uint = 0x01;
pub const VDA_FUNC_DIAG: c_uint = 0x02;
pub const VDA_FUNC_AE: c_uint = 0x03;
pub const VDA_FUNC_CLI: c_uint = 0x04;
pub const VDA_FUNC_IOCTL: c_uint = 0x05;
pub const VDA_FUNC_CFG: c_uint = 0x06;
pub const VDA_FUNC_MGT: c_uint = 0x07;
pub const VDA_FUNC_GSV: c_uint = 0x08;
// VDA request status values.  for host driver considerations, values for
// SCSI requests start at zero.  other requests may use these values as well.
pub const RS_SUCCESS: c_uint = 0x00        /*! successful completion            */;
pub const RS_INV_FUNC: c_uint = 0x01        /*! invalid command function         */;
pub const RS_BUSY: c_uint = 0x02        /*! insufficient resources           */;
pub const RS_SEL: c_uint = 0x03        /*! no target at target_id           */;
pub const RS_NO_LUN: c_uint = 0x04        /*! invalid LUN                      */;
pub const RS_TIMEOUT: c_uint = 0x05        /*! request timeout                  */;
pub const RS_OVERRUN: c_uint = 0x06        /*! data overrun                     */;
pub const RS_UNDERRUN: c_uint = 0x07        /*! data underrun                    */;
pub const RS_SCSI_ERROR: c_uint = 0x08        /*! SCSI error occurred              */;
pub const RS_ABORTED: c_uint = 0x0A        /*! command aborted                  */;
pub const RS_RESID_MISM: c_uint = 0x0B        /*! residual length incorrect        */;
pub const RS_TM_FAILED: c_uint = 0x0C        /*! task management failed           */;
pub const RS_RESET: c_uint = 0x0D        /*! aborted due to bus reset         */;
pub const RS_ERR_DMA_SG: c_uint = 0x0E        /*! error reading SG list            */;
pub const RS_ERR_DMA_DATA: c_uint = 0x0F        /*! error transferring data          */;
pub const RS_UNSUPPORTED: c_uint = 0x10        /*! unsupported request              */;
pub const RS_SEL2: c_uint = 0x70        /*! internal generated RS_SEL        */;
pub const RS_VDA_BASE: c_uint = 0x80        /*! base of VDA-specific errors      */;
pub const RS_MGT_BASE: c_uint = 0x80        /*! base of VDA management errors    */;

pub const RS_FLS_BASE: c_uint = 0xB0        /*! base of VDA errors               */;

pub const RS_CFG_BASE: c_uint = 0xC0        /*! base of VDA configuration errors */;

pub const RS_DEGRADED: c_uint = 0xFB        /*! degraded mode                    */;
pub const RS_CLI_INTERNAL: c_uint = 0xFC        /*! VDA CLI internal error           */;
pub const RS_VDA_INTERNAL: c_uint = 0xFD        /*! catch-all                        */;
pub const RS_PENDING: c_uint = 0xFE        /*! pending, not started             */;
pub const RS_STARTED: c_uint = 0xFF        /*! started                          */;
// flash request subfunctions.  these are used in both the IOCTL and the
// driver-firmware interface (VDA_FUNC_FLASH).
pub const VDA_FLASH_BEGINW: c_uint = 0x00;
pub const VDA_FLASH_READ: c_uint = 0x01;
pub const VDA_FLASH_WRITE: c_uint = 0x02;
pub const VDA_FLASH_COMMIT: c_uint = 0x03;
pub const VDA_FLASH_CANCEL: c_uint = 0x04;
pub const VDA_FLASH_INFO: c_uint = 0x05;
pub const VDA_FLASH_FREAD: c_uint = 0x06;
pub const VDA_FLASH_FWRITE: c_uint = 0x07;
pub const VDA_FLASH_FINFO: c_uint = 0x08;
// IOCTL request subfunctions.  these identify the payload type for
// VDA_FUNC_IOCTL.
//
pub const VDA_IOCTL_HBA: c_uint = 0x00;
pub const VDA_IOCTL_CSMI: c_uint = 0x01;
pub const VDA_IOCTL_SMP: c_uint = 0x02;
pub const VDADEVSTAT_INVALID: c_uint = 0x00;

pub const VDADEVSTAT_ASSIGNED: c_uint = 0x01;
pub const VDADEVSTAT_SPARE: c_uint = 0x02;
pub const VDADEVSTAT_UNAVAIL: c_uint = 0x03;
pub const VDADEVSTAT_PT_MAINT: c_uint = 0x04;
pub const VDADEVSTAT_LCLSPARE: c_uint = 0x05;
pub const VDADEVSTAT_UNUSEABLE: c_uint = 0x06;
pub const VDADEVSTAT_AVAIL: c_uint = 0xFF;
pub const VDA_DEV_OP_CTRL_START: c_uint = 0x01;
pub const VDA_DEV_OP_CTRL_HALT: c_uint = 0x02;
pub const VDA_DEV_OP_CTRL_RESUME: c_uint = 0x03;
pub const VDA_DEV_OP_CTRL_CANCEL: c_uint = 0x04;
pub const VDAMBRSTATE_ONLINE: c_uint = 0x00;
pub const VDAMBRSTATE_DEGRADED: c_uint = 0x01;
pub const VDAMBRSTATE_UNAVAIL: c_uint = 0x02;
pub const VDAMBRSTATE_FAULTED: c_uint = 0x03;
pub const VDAMBRSTATE_MISREAD: c_uint = 0x04;
pub const VDAMBRSTATE_INCOMPAT: c_uint = 0x05;
pub const VDAOP_NONE: c_uint = 0x00;
pub const VDAOP_REBUILD: c_uint = 0x01;
pub const VDAOP_ERASE: c_uint = 0x02;
pub const VDAOP_PATTERN: c_uint = 0x03;
pub const VDAOP_CONVERSION: c_uint = 0x04;
pub const VDAOP_FULL_INIT: c_uint = 0x05;
pub const VDAOP_QUICK_INIT: c_uint = 0x06;
pub const VDAOP_SECT_SCAN: c_uint = 0x07;
pub const VDAOP_SECT_SCAN_PARITY: c_uint = 0x08;
pub const VDAOP_SECT_SCAN_PARITY_FIX: c_uint = 0x09;
pub const VDAOP_RECOV_REBUILD: c_uint = 0x0A;
pub const VDAOPSTAT_OK: c_uint = 0x00;
pub const VDAOPSTAT_FAULTED: c_uint = 0x01;
pub const VDAOPSTAT_HALTED: c_uint = 0x02;
pub const VDAOPSTAT_INT: c_uint = 0x03;
pub const VDASESDI_INVALID: c_uint = 0xFFFF;
pub const VDATGTID_INVALID: c_uint = 0xFFFF;
pub const VDADEVFEAT_ENC_SERV: c_uint = 0x0001;
pub const VDADEVFEAT_IDENT: c_uint = 0x0002;
pub const VDADEVFEAT_DH_SUPP: c_uint = 0x0004;
pub const VDADEVFEAT_PHYS_ID: c_uint = 0x0008;
pub const VDALINKSPEED_UNKNOWN: c_uint = 0x00;
pub const VDALINKSPEED_1GB: c_uint = 0x01;
pub const VDALINKSPEED_1_5GB: c_uint = 0x02;
pub const VDALINKSPEED_2GB: c_uint = 0x03;
pub const VDALINKSPEED_3GB: c_uint = 0x04;
pub const VDALINKSPEED_4GB: c_uint = 0x05;
pub const VDALINKSPEED_6GB: c_uint = 0x06;
pub const VDALINKSPEED_8GB: c_uint = 0x07;
// ! struct atto_vda_devinfo2 is a replacement for atto_vda_devinfo.  it
// extends beyond the 0x70 bytes allowed in atto_vda_mgmt_req; therefore,
// the entire structure is DMaed between the firmware and host buffer and
// the data will always be in little endian format.
//
// This is where fields specific to struct atto_vda_devinfo2 begin.  Note
// that the structure version started at one so applications that unionize this
// structure with atto_vda_dev_info can differentiate them if desired.
//
pub const VDADEVINFO_VERSION0: c_uint = 0x00;
pub const VDADEVINFO_VERSION1: c_uint = 0x01;
pub const VDADEVINFO_VERSION2: c_uint = 0x02;
pub const VDADEVINFO_VERSION3: c_uint = 0x03;

// sector scanning fields
// grp_name was added in version 2 of this structure.
// dev_addr_list was added in version 3 of this structure.
pub const VDA_MAX_RAID_GROUPS: c_int = 32;
pub const VDA_GRP_TYPE_RAID0: c_int = 0;
pub const VDA_GRP_TYPE_RAID1: c_int = 1;
pub const VDA_GRP_TYPE_RAID4: c_int = 4;
pub const VDA_GRP_TYPE_RAID5: c_int = 5;
pub const VDA_GRP_TYPE_RAID6: c_int = 6;
pub const VDA_GRP_TYPE_RAID10: c_int = 10;
pub const VDA_GRP_TYPE_RAID40: c_int = 40;
pub const VDA_GRP_TYPE_RAID50: c_int = 50;
pub const VDA_GRP_TYPE_RAID60: c_int = 60;
pub const VDA_GRP_TYPE_DVRAID_HS: c_int = 252;
pub const VDA_GRP_TYPE_DVRAID_NOHS: c_int = 253;
pub const VDA_GRP_TYPE_JBOD: c_int = 254;
pub const VDA_GRP_TYPE_SPARE: c_int = 255;
pub const VDA_GRP_STAT_INVALID: c_uint = 0x00;
pub const VDA_GRP_STAT_NEW: c_uint = 0x01;
pub const VDA_GRP_STAT_WAITING: c_uint = 0x02;
pub const VDA_GRP_STAT_ONLINE: c_uint = 0x03;
pub const VDA_GRP_STAT_DEGRADED: c_uint = 0x04;
pub const VDA_GRP_STAT_OFFLINE: c_uint = 0x05;
pub const VDA_GRP_STAT_DELETED: c_uint = 0x06;
pub const VDA_GRP_STAT_RECOV_BASIC: c_uint = 0x07;
pub const VDA_GRP_STAT_RECOV_EXTREME: c_uint = 0x08;
pub const VDA_GRP_OP_CTRL_START: c_uint = 0x01;
pub const VDA_GRP_OP_CTRL_HALT: c_uint = 0x02;
pub const VDA_GRP_OP_CTRL_RESUME: c_uint = 0x03;
pub const VDA_GRP_OP_CTRL_CANCEL: c_uint = 0x04;
pub const VDA_RBLD_NONE: c_uint = 0x00;
pub const VDA_RBLD_REBUILD: c_uint = 0x01;
pub const VDA_RBLD_ERASE: c_uint = 0x02;
pub const VDA_RBLD_PATTERN: c_uint = 0x03;
pub const VDA_RBLD_CONV: c_uint = 0x04;
pub const VDA_RBLD_FULL_INIT: c_uint = 0x05;
pub const VDA_RBLD_QUICK_INIT: c_uint = 0x06;
pub const VDA_RBLD_SECT_SCAN: c_uint = 0x07;
pub const VDA_RBLD_SECT_SCAN_PARITY: c_uint = 0x08;
pub const VDA_RBLD_SECT_SCAN_PARITY_FIX: c_uint = 0x09;
pub const VDA_RBLD_RECOV_REBUILD: c_uint = 0x0A;
pub const VDA_RBLD_RECOV_BASIC: c_uint = 0x0B;
pub const VDA_RBLD_RECOV_EXTREME: c_uint = 0x0C;
pub const VDA_MEMBER_MISSING: c_uint = 0xFFFF;
pub const VDA_MEMBER_NEW: c_uint = 0xFFFE;
pub const VDA_GRP_FEAT_HOTSWAP: c_uint = 0x0001;
pub const VDA_GRP_FEAT_SPDRD_MASK: c_uint = 0x0006;
pub const VDA_GRP_FEAT_SPDRD_DIS: c_uint = 0x0000;
pub const VDA_GRP_FEAT_SPDRD_ENB: c_uint = 0x0002;
pub const VDA_GRP_FEAT_SPDRD_AUTO: c_uint = 0x0004;
pub const VDA_GRP_FEAT_IDENT: c_uint = 0x0008;
pub const VDA_GRP_FEAT_RBLDPRI_MASK: c_uint = 0x0030;
pub const VDA_GRP_FEAT_RBLDPRI_LOW: c_uint = 0x0010;
pub const VDA_GRP_FEAT_RBLDPRI_SAME: c_uint = 0x0020;
pub const VDA_GRP_FEAT_RBLDPRI_HIGH: c_uint = 0x0030;
pub const VDA_GRP_FEAT_WRITE_CACHE: c_uint = 0x0040;
pub const VDA_GRP_FEAT_RBLD_RESUME: c_uint = 0x0080;
pub const VDA_GRP_FEAT_SECT_RESUME: c_uint = 0x0100;
pub const VDA_GRP_FEAT_INIT_RESUME: c_uint = 0x0200;
pub const VDA_GRP_FEAT_SSD: c_uint = 0x0400;
pub const VDA_GRP_FEAT_BOOT_DEV: c_uint = 0x0800;
//
// for backward compatibility, a prefetch value of zero means the
// setting is ignored/unsupported.  therefore, the firmware supported
// 0-6 values are incremented to 1-7.
//
pub const VDAGRPOPSTAT_MASK: c_uint = 0x0F;
pub const VDAGRPOPSTAT_INVALID: c_uint = 0x00;
pub const VDAGRPOPSTAT_OK: c_uint = 0x01;
pub const VDAGRPOPSTAT_FAULTED: c_uint = 0x02;
pub const VDAGRPOPSTAT_HALTED: c_uint = 0x03;
pub const VDAGRPOPSTAT_INT: c_uint = 0x04;
pub const VDAGRPOPPROC_MASK: c_uint = 0xF0;
pub const VDAGRPOPPROC_STARTABLE: c_uint = 0x10;
pub const VDAGRPOPPROC_CANCELABLE: c_uint = 0x20;
pub const VDAGRPOPPROC_RESUMABLE: c_uint = 0x40;
pub const VDAGRPOPPROC_HALTABLE: c_uint = 0x80;
pub const VDA_MAX_PARTITIONS: c_int = 128;
pub const VDAPI_FEAT_WRITE_CACHE: c_uint = 0x01;
pub const VDADH_RQTYPE_CACHE: c_uint = 0x01;
pub const VDADH_RQTYPE_FETCH: c_uint = 0x02;
pub const VDADH_RQTYPE_SET_STAT: c_uint = 0x03;
pub const VDADH_RQTYPE_GET_STAT: c_uint = 0x04;
pub const VDADH_RQQUAL_SMART: c_uint = 0x01;
pub const VDADH_RQQUAL_MEDDEF: c_uint = 0x02;
pub const VDADH_RQQUAL_INFOEXC: c_uint = 0x04;
pub const VDADH_STAT_DISABLE: c_uint = 0x00;
pub const VDADH_STAT_ENABLE: c_uint = 0x01;
pub const VDADH_SMARTSTAT_OK: c_uint = 0x00;
pub const VDADH_SMARTSTAT_ERR: c_uint = 0x01;
pub const VDADHSM_RAWSTAT_PREFAIL_WARRANTY: c_uint = 0x01;
pub const VDADHSM_RAWSTAT_ONLINE_COLLECTION: c_uint = 0x02;
pub const VDADHSM_RAWSTAT_PERFORMANCE_ATTR: c_uint = 0x04;
pub const VDADHSM_RAWSTAT_ERROR_RATE_ATTR: c_uint = 0x08;
pub const VDADHSM_RAWSTAT_EVENT_COUNT_ATTR: c_uint = 0x10;
pub const VDADHSM_RAWSTAT_SELF_PRESERVING_ATTR: c_uint = 0x20;
pub const VDADHSM_CALCSTAT_UNKNOWN: c_uint = 0x00;
pub const VDADHSM_CALCSTAT_GOOD: c_uint = 0x01;
pub const VDADHSM_CALCSTAT_PREFAIL: c_uint = 0x02;
pub const VDADHSM_CALCSTAT_OLDAGE: c_uint = 0x03;
pub const VDAMET_VERSION0: c_uint = 0x00;

pub const VDAMET_METACT_NONE: c_uint = 0x00;
pub const VDAMET_METACT_START: c_uint = 0x01;
pub const VDAMET_METACT_STOP: c_uint = 0x02;
pub const VDAMET_METACT_RETRIEVE: c_uint = 0x03;
pub const VDAMET_METACT_CLEAR: c_uint = 0x04;
pub const VDAMET_TSTACT_NONE: c_uint = 0x00;
pub const VDAMET_TSTACT_STRT_INIT: c_uint = 0x01;
pub const VDAMET_TSTACT_STRT_READ: c_uint = 0x02;
pub const VDAMET_TSTACT_STRT_VERIFY: c_uint = 0x03;
pub const VDAMET_TSTACT_STRT_INIT_VERIFY: c_uint = 0x04;
pub const VDAMET_TSTACT_STOP: c_uint = 0x05;
pub const VDAMET_ALL_DEVICES: c_uint = 0xFF;
pub const VDAMD_LEN_LAST: c_uint = 0x8000;
pub const VDAMD_LEN_MASK: c_uint = 0x0FFF;
pub const VDAMDF_RUN: c_uint = 0x00000007;
pub const VDAMDF_RUN_READ: c_uint = 0x00000001;
pub const VDAMDF_RUN_WRITE: c_uint = 0x00000002;
pub const VDAMDF_RUN_ALL: c_uint = 0x00000004;
pub const VDAMDF_READ: c_uint = 0x00000010;
pub const VDAMDF_WRITE: c_uint = 0x00000020;
pub const VDAMDF_ALL: c_uint = 0x00000040;
pub const VDAMDF_DRIVETEST: c_uint = 0x40000000;
pub const VDAMDF_NEW: c_uint = 0x80000000;
pub const VDASI_SCHTYPE_ONETIME: c_uint = 0x01;
pub const VDASI_SCHTYPE_DAILY: c_uint = 0x02;
pub const VDASI_SCHTYPE_WEEKLY: c_uint = 0x03;
pub const VDASI_OP_NONE: c_uint = 0x00;
pub const VDASI_OP_CREATE: c_uint = 0x01;
pub const VDASI_OP_CANCEL: c_uint = 0x02;
pub const VDASI_DAY_NONE: c_uint = 0x00;
pub const VDASI_PROG_NONE: c_uint = 0xFF;
pub const VDASI_EVTTYPE_SECT_SCAN: c_uint = 0x01;
pub const VDASI_EVTTYPE_SECT_SCAN_PARITY: c_uint = 0x02;
pub const VDASI_EVTTYPE_SECT_SCAN_PARITY_FIX: c_uint = 0x03;
pub const VDASI_RECUR_FOREVER: c_uint = 0x00;
pub const VDASI_ID_NONE: c_uint = 0x00;
pub const VDANVCI_SUPERCAP_NOT_PRESENT: c_uint = 0x00;
pub const VDANVCI_SUPERCAP_FULLY_CHARGED: c_uint = 0x01;
pub const VDANVCI_SUPERCAP_NOT_CHARGED: c_uint = 0x02;
pub const VDANVCI_NVCACHEMODULE_NOT_PRESENT: c_uint = 0x00;
pub const VDANVCI_NVCACHEMODULE_PRESENT: c_uint = 0x01;
pub const VDANVCI_PROTMODE_HI_PROTECT: c_uint = 0x00;
pub const VDANVCI_PROTMODE_HI_PERFORM: c_uint = 0x01;
pub const VDABUZZI_BUZZER_OFF: c_uint = 0x00;
pub const VDABUZZI_BUZZER_ON: c_uint = 0x01;
pub const VDABUZZI_BUZZER_LAST: c_uint = 0x02;
pub const VDABUZZI_DURATION_INDEFINITE: c_uint = 0xffffffff;
pub const VDAADAPINFO_VERSION0: c_uint = 0x00;

pub const VDA_ADAP_FEAT_IDENT: c_uint = 0x0001;
pub const VDA_ADAP_FEAT_BUZZ_ERR: c_uint = 0x0002;
pub const VDA_ADAP_FEAT_UTC_TIME: c_uint = 0x0004;
pub const VDA_TEMP_TYPE_CPU: c_int = 1;
pub const VDA_FAN_STAT_UNKNOWN: c_int = 0;
pub const VDA_FAN_STAT_NORMAL: c_int = 1;
pub const VDA_FAN_STAT_FAIL: c_int = 2;
// VDA management commands
pub const VDAMGT_DEV_SCAN: c_uint = 0x00;
pub const VDAMGT_DEV_INFO: c_uint = 0x01;
pub const VDAMGT_DEV_CLEAN: c_uint = 0x02;
pub const VDAMGT_DEV_IDENTIFY: c_uint = 0x03;
pub const VDAMGT_DEV_IDENTSTOP: c_uint = 0x04;
pub const VDAMGT_DEV_PT_INFO: c_uint = 0x05;
pub const VDAMGT_DEV_FEATURES: c_uint = 0x06;
pub const VDAMGT_DEV_PT_FEATURES: c_uint = 0x07;
pub const VDAMGT_DEV_HEALTH_REQ: c_uint = 0x08;
pub const VDAMGT_DEV_METRICS: c_uint = 0x09;
pub const VDAMGT_DEV_INFO2: c_uint = 0x0A;
pub const VDAMGT_DEV_OPERATION: c_uint = 0x0B;
pub const VDAMGT_DEV_INFO2_BYADDR: c_uint = 0x0C;
pub const VDAMGT_GRP_INFO: c_uint = 0x10;
pub const VDAMGT_GRP_CREATE: c_uint = 0x11;
pub const VDAMGT_GRP_DELETE: c_uint = 0x12;
pub const VDAMGT_ADD_STORAGE: c_uint = 0x13;
pub const VDAMGT_MEMBER_ADD: c_uint = 0x14;
pub const VDAMGT_GRP_COMMIT: c_uint = 0x15;
pub const VDAMGT_GRP_REBUILD: c_uint = 0x16;
pub const VDAMGT_GRP_COMMIT_INIT: c_uint = 0x17;
pub const VDAMGT_QUICK_RAID: c_uint = 0x18;
pub const VDAMGT_GRP_FEATURES: c_uint = 0x19;
pub const VDAMGT_GRP_COMMIT_INIT_AUTOMAP: c_uint = 0x1A;
pub const VDAMGT_QUICK_RAID_INIT_AUTOMAP: c_uint = 0x1B;
pub const VDAMGT_GRP_OPERATION: c_uint = 0x1C;
pub const VDAMGT_CFG_SAVE: c_uint = 0x20;
pub const VDAMGT_LAST_ERROR: c_uint = 0x21;
pub const VDAMGT_ADAP_INFO: c_uint = 0x22;
pub const VDAMGT_ADAP_FEATURES: c_uint = 0x23;
pub const VDAMGT_TEMP_INFO: c_uint = 0x24;
pub const VDAMGT_FAN_INFO: c_uint = 0x25;
pub const VDAMGT_PART_INFO: c_uint = 0x30;
pub const VDAMGT_PART_MAP: c_uint = 0x31;
pub const VDAMGT_PART_UNMAP: c_uint = 0x32;
pub const VDAMGT_PART_AUTOMAP: c_uint = 0x33;
pub const VDAMGT_PART_SPLIT: c_uint = 0x34;
pub const VDAMGT_PART_MERGE: c_uint = 0x35;
pub const VDAMGT_SPARE_LIST: c_uint = 0x40;
pub const VDAMGT_SPARE_ADD: c_uint = 0x41;
pub const VDAMGT_SPARE_REMOVE: c_uint = 0x42;
pub const VDAMGT_LOCAL_SPARE_ADD: c_uint = 0x43;
pub const VDAMGT_SCHEDULE_EVENT: c_uint = 0x50;
pub const VDAMGT_SCHEDULE_INFO: c_uint = 0x51;
pub const VDAMGT_NVCACHE_INFO: c_uint = 0x60;
pub const VDAMGT_NVCACHE_SET: c_uint = 0x61;
pub const VDAMGT_BUZZER_INFO: c_uint = 0x70;
pub const VDAMGT_BUZZER_SET: c_uint = 0x71;
pub const VDAAE_HDRF_EVENT_ACK: c_uint = 0x01;
pub const VDAAE_HDR_VER_0: c_int = 0;
pub const VDAAE_HDR_TYPE_RAID: c_int = 1;
pub const VDAAE_HDR_TYPE_LU: c_int = 2;
pub const VDAAE_HDR_TYPE_DISK: c_int = 3;
pub const VDAAE_HDR_TYPE_RESET: c_int = 4;
pub const VDAAE_HDR_TYPE_LOG_INFO: c_int = 5;
pub const VDAAE_HDR_TYPE_LOG_WARN: c_int = 6;
pub const VDAAE_HDR_TYPE_LOG_CRIT: c_int = 7;
pub const VDAAE_HDR_TYPE_LOG_FAIL: c_int = 8;
pub const VDAAE_HDR_TYPE_NVC: c_int = 9;
pub const VDAAE_HDR_TYPE_TLG_INFO: c_int = 10;
pub const VDAAE_HDR_TYPE_TLG_WARN: c_int = 11;
pub const VDAAE_HDR_TYPE_TLG_CRIT: c_int = 12;
pub const VDAAE_HDR_TYPE_PWRMGT: c_int = 13;
pub const VDAAE_HDR_TYPE_MUTE: c_int = 14;
pub const VDAAE_HDR_TYPE_DEV: c_int = 15;
pub const VDAAE_GROUP_STATE: c_uint = 0x00000001;
pub const VDAAE_RBLD_STATE: c_uint = 0x00000002;
pub const VDAAE_RBLD_PROG: c_uint = 0x00000004;
pub const VDAAE_MEMBER_CHG: c_uint = 0x00000008;
pub const VDAAE_PART_CHG: c_uint = 0x00000010;
pub const VDAAE_MEM_STATE_CHG: c_uint = 0x00000020;
pub const VDAAE_RAID_INVALID: c_int = 0;
pub const VDAAE_RAID_NEW: c_int = 1;
pub const VDAAE_RAID_WAITING: c_int = 2;
pub const VDAAE_RAID_ONLINE: c_int = 3;
pub const VDAAE_RAID_DEGRADED: c_int = 4;
pub const VDAAE_RAID_OFFLINE: c_int = 5;
pub const VDAAE_RAID_DELETED: c_int = 6;
pub const VDAAE_RAID_BASIC: c_int = 7;
pub const VDAAE_RAID_EXTREME: c_int = 8;
pub const VDAAE_RAID_UNKNOWN: c_int = 9;
pub const VDAAE_RBLD_NONE: c_int = 0;
pub const VDAAE_RBLD_REBUILD: c_int = 1;
pub const VDAAE_RBLD_ERASE: c_int = 2;
pub const VDAAE_RBLD_PATTERN: c_int = 3;
pub const VDAAE_RBLD_CONV: c_int = 4;
pub const VDAAE_RBLD_FULL_INIT: c_int = 5;
pub const VDAAE_RBLD_QUICK_INIT: c_int = 6;
pub const VDAAE_RBLD_SECT_SCAN: c_int = 7;
pub const VDAAE_RBLD_SECT_SCAN_PARITY: c_int = 8;
pub const VDAAE_RBLD_SECT_SCAN_PARITY_FIX: c_int = 9;
pub const VDAAE_RBLD_RECOV_REBUILD: c_int = 10;
pub const VDAAE_RBLD_UNKNOWN: c_int = 11;
pub const VDAAE_GRPOPSTAT_MASK: c_uint = 0x0F;
pub const VDAAE_GRPOPSTAT_INVALID: c_uint = 0x00;
pub const VDAAE_GRPOPSTAT_OK: c_uint = 0x01;
pub const VDAAE_GRPOPSTAT_FAULTED: c_uint = 0x02;
pub const VDAAE_GRPOPSTAT_HALTED: c_uint = 0x03;
pub const VDAAE_GRPOPSTAT_INT: c_uint = 0x04;
pub const VDAAE_GRPOPPROC_MASK: c_uint = 0xF0;
pub const VDAAE_GRPOPPROC_STARTABLE: c_uint = 0x10;
pub const VDAAE_GRPOPPROC_CANCELABLE: c_uint = 0x20;
pub const VDAAE_GRPOPPROC_RESUMABLE: c_uint = 0x40;
pub const VDAAE_GRPOPPROC_HALTABLE: c_uint = 0x80;
pub const VDAAE_LU_DISC: c_uint = 0x00000001;
pub const VDAAE_LU_LOST: c_uint = 0x00000002;
pub const VDAAE_LU_STATE: c_uint = 0x00000004;
pub const VDAAE_LU_PASSTHROUGH: c_uint = 0x10000000;
pub const VDAAE_LU_PHYS_ID: c_uint = 0x20000000;
pub const VDAAE_LU_UNDEFINED: c_int = 0;
pub const VDAAE_LU_NOT_PRESENT: c_int = 1;
pub const VDAAE_LU_OFFLINE: c_int = 2;
pub const VDAAE_LU_ONLINE: c_int = 3;
pub const VDAAE_LU_DEGRADED: c_int = 4;
pub const VDAAE_LU_FACTORY_DISABLED: c_int = 5;
pub const VDAAE_LU_DELETED: c_int = 6;
pub const VDAAE_LU_BUSSCAN: c_int = 7;
pub const VDAAE_LU_UNKNOWN: c_int = 8;
pub const VDAAE_LOG_STRSZ: c_int = 64;
pub const VDAAE_TLG_STRSZ: c_int = 56;
#[repr(C)]
#[derive(Copy, Clone)]
pub union atto_vda_ae {
    pub hdr: atto_vda_ae_hdr,
    pub disk: atto_vda_ae_disk,
    pub lu: atto_vda_ae_lu,
    pub raid: atto_vda_ae_raid,
    pub log: atto_vda_ae_log,
    pub tslog: atto_vda_ae_timestamp_log,
    pub nvcache: atto_vda_ae_nvc,
    pub dev: atto_vda_ae_dev,
}

pub const VDA_DT_DAY_MASK: c_uint = 0x07;
pub const VDA_DT_DAY_NONE: c_uint = 0x00;
pub const VDA_DT_DAY_SUN: c_uint = 0x01;
pub const VDA_DT_DAY_MON: c_uint = 0x02;
pub const VDA_DT_DAY_TUE: c_uint = 0x03;
pub const VDA_DT_DAY_WED: c_uint = 0x04;
pub const VDA_DT_DAY_THU: c_uint = 0x05;
pub const VDA_DT_DAY_FRI: c_uint = 0x06;
pub const VDA_DT_DAY_SAT: c_uint = 0x07;
pub const VDA_DT_PM: c_uint = 0x40;
pub const VDA_DT_MILITARY: c_uint = 0x80;
pub const SGE_LEN_LIMIT: c_uint = 0x003FFFFF      /*! mask of segment length            */;
pub const SGE_LEN_MAX: c_uint = 0x003FF000      /*! maximum segment length            */;
pub const SGE_LAST: c_uint = 0x01000000      /*! last entry                        */;
pub const SGE_ADDR_64: c_uint = 0x04000000      /*! 64-bit addressing flag            */;
pub const SGE_CHAIN: c_uint = 0x80000000      /*! chain descriptor flag             */;
pub const SGE_CHAIN_LEN: c_uint = 0x0000FFFF      /*! mask of length in chain entries   */;
pub const SGE_CHAIN_SZ: c_uint = 0x00FF0000      /*! mask of size of chained buffer    */;
pub const VDA_ITF_MEM_RW: c_uint = 0x00000001;
pub const VDA_ITF_TRACE: c_uint = 0x00000002;
pub const VDA_ITF_SCSI_PASS_THRU: c_uint = 0x00000004;
pub const VDA_ITF_GET_DEV_ADDR: c_uint = 0x00000008;
pub const VDA_ITF_PHY_CTRL: c_uint = 0x00000010;
pub const VDA_ITF_CONN_CTRL: c_uint = 0x00000020;
pub const VDA_ITF_GET_DEV_INFO: c_uint = 0x00000040;
// configuration commands
pub const VDA_CFG_INIT: c_uint = 0x00;
pub const VDA_CFG_GET_INIT: c_uint = 0x01;
pub const VDA_CFG_GET_INIT2: c_uint = 0x02;
// ! physical region descriptor (PRD) aka scatter/gather entry
pub const PRD_LEN_LIMIT: c_uint = 0x003FFFFF;
pub const PRD_LEN_MAX: c_uint = 0x003FF000;
pub const PRD_NXT_PRD_CNT: c_uint = 0x0000007F;
pub const PRD_CHAIN: c_uint = 0x01000000;
pub const PRD_DATA: c_uint = 0x00000000;
pub const PRD_INT_SEL: c_uint = 0xF0000000;
pub const PRD_INT_SEL_F0: c_uint = 0x00000000;
pub const PRD_INT_SEL_F1: c_uint = 0x40000000;
pub const PRD_INT_SEL_F2: c_uint = 0x80000000;
pub const PRD_INT_SEL_F3: c_uint = 0xc0000000;
pub const PRD_INT_SEL_SRAM: c_uint = 0x10000000;
pub const PRD_INT_SEL_PBSR: c_uint = 0x20000000;
// Request types. NOTE that ALL requests have the same layout for the first
// few bytes.
//
pub const FCP_CDB_SIZE: c_int = 16;
pub const FCP_CMND_LUN_MASK: c_uint = 0x000000FF;
pub const FCP_CMND_TA_MASK: c_uint = 0x00000700;
pub const FCP_CMND_TA_SIMPL_Q: c_uint = 0x00000000;
pub const FCP_CMND_TA_HEAD_Q: c_uint = 0x00000100;
pub const FCP_CMND_TA_ORDRD_Q: c_uint = 0x00000200;
pub const FCP_CMND_TA_ACA: c_uint = 0x00000400;
pub const FCP_CMND_PRI_MASK: c_uint = 0x00007800;
pub const FCP_CMND_TM_MASK: c_uint = 0x00FF0000;
pub const FCP_CMND_ATS: c_uint = 0x00020000;
pub const FCP_CMND_CTS: c_uint = 0x00040000;
pub const FCP_CMND_LRS: c_uint = 0x00100000;
pub const FCP_CMND_TRS: c_uint = 0x00200000;
pub const FCP_CMND_CLA: c_uint = 0x00400000;
pub const FCP_CMND_TRM: c_uint = 0x00800000;
pub const FCP_CMND_DATA_DIR: c_uint = 0x03000000;
pub const FCP_CMND_WRD: c_uint = 0x01000000;
pub const FCP_CMND_RDD: c_uint = 0x02000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub ppsense_buf: u64,
    pub target_id: u16,
    pub iblk_cnt_prd: u8,
    pub reserved: u8,
}

pub const VDA_DIAG_STATUS: c_uint = 0x00;
pub const VDA_DIAG_RESET: c_uint = 0x01;
pub const VDA_DIAG_PAUSE: c_uint = 0x02;
pub const VDA_DIAG_RESUME: c_uint = 0x03;
pub const VDA_DIAG_READ: c_uint = 0x04;
pub const VDA_DIAG_WRITE: c_uint = 0x05;
#[repr(C)]
#[derive(Copy, Clone)]
pub union atto_vda_req {
    pub scsi: atto_vda_scsi_req,
    pub flash: atto_vda_flash_req,
    pub diag: atto_vda_diag_req,
    pub ae: atto_vda_ae_req,
    pub cli: atto_vda_cli_req,
    pub ioctl: atto_vda_ioctl_req,
    pub cfg: atto_vda_cfg_req,
    pub mgt: atto_vda_mgmt_req,
    pub bytes: [u8; 1024],
}

// Outbound response structures
#[repr(C)]
#[derive(Copy, Clone)]
pub union atto_vda_func_rsp {
    pub scsi_rsp: atto_vda_scsi_rsp,
    pub flash_rsp: atto_vda_flash_rsp,
    pub ae_rsp: atto_vda_ae_rsp,
    pub cli_rsp: atto_vda_cli_rsp,
    pub ioctl_rsp: atto_vda_ioctl_rsp,
    pub cfg_rsp: atto_vda_cfg_rsp,
    pub mgt_rsp: atto_vda_mgmt_rsp,
    pub dwords: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union atto_vda_rsp_data {
    pub ae_data: atto_vda_ae_data,
    pub mgt_data: atto_vda_mgmt_data,
    pub sense_data: [u8; 252],    pub bytes: [u8; 256],
}
