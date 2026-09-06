//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvif/if0012.h
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


// SPDX-License-Identifier: MIT

#[repr(C)]
#[derive(Copy, Clone)]
pub union nvif_outp_args {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_outp_v0 {
    pub version: __u8,
    pub /: *mut *mut __u8 id; / DCB device index.,
pub const NVIF_OUTP_V0_TYPE_DAC: c_uint = 0x00;
pub const NVIF_OUTP_V0_TYPE_SOR: c_uint = 0x01;
pub const NVIF_OUTP_V0_TYPE_PIOR: c_uint = 0x02;
    pub type: __u8,
pub const NVIF_OUTP_V0_PROTO_RGB_CRT: c_uint = 0x00;
pub const NVIF_OUTP_V0_PROTO_TMDS: c_uint = 0x01;
pub const NVIF_OUTP_V0_PROTO_LVDS: c_uint = 0x02;
pub const NVIF_OUTP_V0_PROTO_DP: c_uint = 0x03;
    pub proto: __u8,
    pub heads: __u8,
    pub ddc: __u8,
    pub conn: __u8,
    pub freq_max: __u32,
    pub rgb_crt: },
    pub dual: __u8,
    pub tmds: },
    pub acpi_edid: __u8,
    pub lvds: },
    pub aux: __u8,
    pub mst: __u8,
    pub increased_wm: __u8,
    pub link_nr: __u8,
    pub link_bw: __u32,
    pub dp: },
}

pub const NVIF_OUTP_V0_DETECT: c_uint = 0x00;
pub const NVIF_OUTP_V0_EDID_GET: c_uint = 0x01;
pub const NVIF_OUTP_V0_INHERIT: c_uint = 0x10;
pub const NVIF_OUTP_V0_ACQUIRE: c_uint = 0x11;
pub const NVIF_OUTP_V0_RELEASE: c_uint = 0x12;
pub const NVIF_OUTP_V0_LOAD_DETECT: c_uint = 0x20;
pub const NVIF_OUTP_V0_BL_GET: c_uint = 0x30;
pub const NVIF_OUTP_V0_BL_SET: c_uint = 0x31;
pub const NVIF_OUTP_V0_LVDS: c_uint = 0x40;
pub const NVIF_OUTP_V0_HDMI: c_uint = 0x50;
pub const NVIF_OUTP_V0_INFOFRAME: c_uint = 0x60;
pub const NVIF_OUTP_V0_HDA_ELD: c_uint = 0x61;
pub const NVIF_OUTP_V0_DP_AUX_PWR: c_uint = 0x70;
pub const NVIF_OUTP_V0_DP_AUX_XFER: c_uint = 0x71;
pub const NVIF_OUTP_V0_DP_RATES: c_uint = 0x72;
pub const NVIF_OUTP_V0_DP_TRAIN: c_uint = 0x73;
pub const NVIF_OUTP_V0_DP_DRIVE: c_uint = 0x74;
pub const NVIF_OUTP_V0_DP_SST: c_uint = 0x75;
pub const NVIF_OUTP_V0_DP_MST_ID_GET: c_uint = 0x76;
pub const NVIF_OUTP_V0_DP_MST_ID_PUT: c_uint = 0x77;
pub const NVIF_OUTP_V0_DP_MST_VCPI: c_uint = 0x78;
#[repr(C)]
#[derive(Copy, Clone)]
pub union nvif_outp_detect_args {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_outp_detect_v0 {
    pub version: __u8,
pub const NVIF_OUTP_DETECT_V0_NOT_PRESENT: c_uint = 0x00;
pub const NVIF_OUTP_DETECT_V0_PRESENT: c_uint = 0x01;
pub const NVIF_OUTP_DETECT_V0_UNKNOWN: c_uint = 0x02;
    pub status: __u8,
    pub v0: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nvif_outp_edid_get_args {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_outp_edid_get_v0 {
    pub version: __u8,
    pub pad01: __u8,
    pub size: __u16,
    pub data: [__u8; 2048],
    pub v0: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nvif_outp_load_detect_args {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_outp_load_detect_v0 {
    pub version: __u8,
    pub load: __u8,
    pub pad02: [__u8; 2],
    pub /: *mut *mut __u32 data; /TODO: move vbios loadval parsing into nvkm,
    pub v0: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nvif_outp_acquire_args {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_outp_acquire_v0 {
    pub version: __u8,
pub const NVIF_OUTP_ACQUIRE_V0_DAC: c_uint = 0x00;
pub const NVIF_OUTP_ACQUIRE_V0_SOR: c_uint = 0x01;
pub const NVIF_OUTP_ACQUIRE_V0_PIOR: c_uint = 0x02;
    pub type: __u8,
    pub or: __u8,
    pub link: __u8,
    pub pad04: [__u8; 4],
    pub hda: __u8,
    pub sor: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nvif_outp_inherit_args {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_outp_inherit_v0 {
    pub version: __u8,
pub const NVIF_OUTP_INHERIT_V0_RGB_CRT: c_uint = 0x00;
pub const NVIF_OUTP_INHERIT_V0_TV: c_uint = 0x01;
pub const NVIF_OUTP_INHERIT_V0_TMDS: c_uint = 0x02;
pub const NVIF_OUTP_INHERIT_V0_LVDS: c_uint = 0x03;
pub const NVIF_OUTP_INHERIT_V0_DP: c_uint = 0x04;
// In/out. Input is one of the above values, output is the actual hw protocol
    pub proto: __u8,
    pub or: __u8,
    pub link: __u8,
    pub head: __u8,
// TODO: Figure out padding, and whether we even want this field
    pub hda: __u8,
    pub tmds: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nvif_outp_release_args {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_outp_release_vn {
    pub vn: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nvif_outp_bl_get_args {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_outp_bl_get_v0 {
    pub version: __u8,
    pub level: __u8,
    pub v0: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nvif_outp_bl_set_args {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_outp_bl_set_v0 {
    pub version: __u8,
    pub level: __u8,
    pub v0: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nvif_outp_lvds_args {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_outp_lvds_v0 {
    pub version: __u8,
    pub dual: __u8,
    pub bpc8: __u8,
    pub v0: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nvif_outp_hdmi_args {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_outp_hdmi_v0 {
    pub version: __u8,
    pub head: __u8,
    pub enable: __u8,
    pub max_ac_packet: __u8,
    pub rekey: __u8,
    pub scdc: __u8,
    pub scdc_scrambling: __u8,
    pub scdc_low_rates: __u8,
    pub khz: __u32,
    pub v0: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nvif_outp_infoframe_args {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_outp_infoframe_v0 {
    pub version: __u8,
pub const NVIF_OUTP_INFOFRAME_V0_AVI: c_int = 0;
pub const NVIF_OUTP_INFOFRAME_V0_VSI: c_int = 1;
    pub type: __u8,
    pub head: __u8,
    pub pad03: [__u8; 5],
    pub data: [__u8; ],
    pub v0: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nvif_outp_hda_eld_args {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_outp_hda_eld_v0 {
    pub version: __u8,
    pub head: __u8,
    pub pad02: [__u8; 6],
    pub data: [__u8; ],
    pub v0: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nvif_outp_dp_aux_pwr_args {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_outp_dp_aux_pwr_v0 {
    pub version: __u8,
    pub state: __u8,
    pub pad02: [__u8; 6],
    pub v0: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nvif_outp_dp_aux_xfer_args {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_outp_dp_aux_xfer_v0 {
    pub version: __u8,
    pub pad01: __u8,
    pub type: __u8,
    pub size: __u8,
    pub addr: __u32,
    pub data: [__u8; 16],
    pub v0: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nvif_outp_dp_rates_args {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_outp_dp_rates_v0 {
    pub version: __u8,
    pub pad01: [__u8; 6],
    pub rates: __u8,
    pub dpcd: __s8,
    pub rate: __u32,
    pub rate: [}; 8],
    pub v0: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nvif_outp_dp_train_args {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_outp_dp_train_v0 {
    pub version: __u8,
    pub retrain: __u8,
    pub mst: __u8,
    pub lttprs: __u8,
    pub post_lt_adj: __u8,
    pub link_nr: __u8,
    pub link_bw: __u32,
    pub dpcd: [__u8; DP_RECEIVER_CAP_SIZE],
    pub v0: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nvif_outp_dp_drive_args {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_outp_dp_drive_v0 {
    pub version: __u8,
    pub pad01: [__u8; 2],
    pub lanes: __u8,
    pub pe: [__u8; 4],
    pub vs: [__u8; 4],
    pub v0: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nvif_outp_dp_sst_args {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_outp_dp_sst_v0 {
    pub version: __u8,
    pub head: __u8,
    pub pad02: [__u8; 2],
    pub watermark: __u32,
    pub hblanksym: __u32,
    pub vblanksym: __u32,
    pub v0: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nvif_outp_dp_mst_id_put_args {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_outp_dp_mst_id_put_v0 {
    pub version: __u8,
    pub pad01: [__u8; 3],
    pub id: __u32,
    pub v0: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nvif_outp_dp_mst_id_get_args {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_outp_dp_mst_id_get_v0 {
    pub version: __u8,
    pub pad01: [__u8; 3],
    pub id: __u32,
    pub v0: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nvif_outp_dp_mst_vcpi_args {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_outp_dp_mst_vcpi_v0 {
    pub version: __u8,
    pub head: __u8,
    pub start_slot: __u8,
    pub num_slots: __u8,
    pub pbn: __u16,
    pub aligned_pbn: __u16,
    pub v0: },
}
