
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

use core::num::NonZero;

use kernel::{
    device,
    pci,
    prelude::*, //
};

use crate::{
    fsp::{
        Fsp,
        VgpuMode, //
    },
    gpu::Chipset, //
};

mod hal;

/// vGPU state detected during GPU construction.
#[derive(Debug, Clone, Copy)]
pub(crate) enum VgpuState {
    /// vGPU mode is not enabled for this boot.
    Disabled,
    /// vGPU mode is enabled for this boot.
    Enabled {
        /// Total number of SR-IOV VFs supported by this device.
        total_vfs: NonZero<u16>,
    },
}

/// vGPU state manager.
pub(crate) struct VgpuManager {
    state: VgpuState,
}

impl VgpuManager {
    /// Creates a vGPU manager by querying SR-IOV and the FSP PRC vGPU knob.
    pub(crate) fn new(
        pdev: &pci::Device<device::Core<'_>>,
        chipset: Chipset,
        fsp: Option<&mut Fsp<'_>>,
    ) -> Self {
        let state = Self::detect_state(pdev, chipset, fsp).unwrap_or_else(|e| {
            dev_warn!(
                pdev,
                "vGPU state detection failed: {:?}; disabling vGPU\n",
                e
            );
            VgpuState::Disabled
        });
        dev_dbg!(pdev, "vGPU state: {:?}\n", state);

        Self { state }
    }

    /// Detects the vGPU state from the chipset, SR-IOV capability and FSP PRC knob.
    fn detect_state(
        pdev: &pci::Device<device::Core<'_>>,
        chipset: Chipset,
        fsp: Option<&mut Fsp<'_>>,
    ) -> Result<VgpuState> {
        if !hal::vgpu_hal(chipset).supports_vgpu() {
            return Ok(VgpuState::Disabled);
        }

        let Some(total_vfs) = pdev.sriov_get_totalvfs() else {
            return Ok(VgpuState::Disabled);
        };

        if total_vfs.get() < 2 {
            // The current vGPU path does not support single-VF SR-IOV devices yet.
            // Treat one total VF as vGPU-disabled for now; single-VF support can relax
            // this gate once the manager handles that topology.
            return Ok(VgpuState::Disabled);
        }

        let fsp = fsp.ok_or(ENODEV)?;

        match fsp.read_vgpu_mode(pdev.as_ref())? {
            VgpuMode::Enabled => Ok(VgpuState::Enabled { total_vfs }),
            VgpuMode::Disabled => Ok(VgpuState::Disabled),
        }
    }

    /// Returns the detected vGPU state for this boot.
    pub(crate) fn state(&self) -> VgpuState {
        self.state
    }
}
