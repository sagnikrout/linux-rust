
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

use core::marker::PhantomData;

use kernel::{
    io::{
        poll::read_poll_timeout,
        register::WithBase,
        Io, //
    },
    prelude::*,
    time::Delta, //
};

use crate::{
    falcon::{
        hal::LoadMethod,
        Falcon,
        FalconBromParams,
        FalconEngine, //
    },
    regs, //
};

use super::FalconHal;

pub(super) struct Tu102<E: FalconEngine>(PhantomData<E>);

impl<E: FalconEngine> Tu102<E> {
    pub(super) fn new() -> Self {
        Self(PhantomData)
    }
}

impl<E: FalconEngine> FalconHal<E> for Tu102<E> {
    fn select_core(&self, _falcon: &Falcon<'_, E>) -> Result {
        Ok(())
    }

    fn signature_reg_fuse_version(
        &self,
        _falcon: &Falcon<'_, E>,
        _engine_id_mask: u16,
        _ucode_id: u8,
    ) -> Result<u32> {
        Ok(0)
    }

    fn program_brom(&self, _falcon: &Falcon<'_, E>, _params: &FalconBromParams) {}

    fn is_riscv_active(&self, falcon: &Falcon<'_, E>) -> bool {
        falcon
            .bar
            .read(regs::NV_PRISCV_RISCV_CORE_SWITCH_RISCV_STATUS::of::<E>())
            .active_stat()
    }

    fn is_riscv_halted(&self, _falcon: &Falcon<'_, E>) -> Result<bool> {
        Err(ENOTSUPP)
    }

    fn reset_wait_mem_scrubbing(&self, falcon: &Falcon<'_, E>) -> Result {
        // TIMEOUT: memory scrubbing should complete in less than 10ms.
        read_poll_timeout(
            || Ok(falcon.bar.read(regs::NV_PFALCON_FALCON_DMACTL::of::<E>())),
            |r| r.mem_scrubbing_done(),
            Delta::ZERO,
            Delta::from_millis(10),
        )
        .map(|_| ())
    }

    fn reset_eng(&self, falcon: &Falcon<'_, E>) -> Result {
        regs::NV_PFALCON_FALCON_ENGINE::reset_engine::<E>(falcon.bar);
        self.reset_wait_mem_scrubbing(falcon)?;

        Ok(())
    }

    fn load_method(&self) -> LoadMethod {
        LoadMethod::Pio
    }
}
