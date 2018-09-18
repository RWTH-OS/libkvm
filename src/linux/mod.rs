// Copyright (C) 2018, Allison Randal
//
// Licensed under LGPL version 2 or any later version.

//! Constants and structs for interfacing with the KVM API.
//!
//! These are defined in Rust, but mimic the C constants and structs
//! defined in `ioctl.h` and `linux/kvm.h`.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod kvm_ioctl;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod x86_kvm_bindings;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use self::x86_kvm_bindings as kvm_bindings;

use std::fmt;
use linux::x86_kvm_bindings::{kvm_regs,kvm_sregs,kvm_dtable,kvm_segment};

impl fmt::Display for kvm_regs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
	            "rip: {:016x}   rsp: {:016x} flags: {:016x}\n\
	            rax: {:016x}   rbx: {:016x}   rcx: {:016x}\n\
	            rdx: {:016x}   rsi: {:016x}   rdi: {:016x}\n\
	            rbp: {:016x}    r8: {:016x}    r9: {:016x}\n\
	            r10: {:016x}   r11: {:016x}   r12: {:016x}\n\
	            r13: {:016x}   r14: {:016x}   r15: {:016x}\n",
	            self.rip, self.rsp, self.rflags,
	            self.rax, self.rbx, self.rcx,
	            self.rdx, self.rsi, self.rdi,
	            self.rbp, self.r8,  self.r9,
	            self.r10, self.r11, self.r12,
	            self.r13, self.r14, self.r15
        )
    }
}

impl fmt::Display for kvm_sregs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"cr0: {:016x}   cr2: {:016x}   cr3: {:016x}\ncr4: {:016x}   cr8: {:016x}\n",
            self.cr0, self.cr2, self.cr3, self.cr4, self.cr8)
    }
}

impl fmt::Display for kvm_dtable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{:016x}  {:08x}", self.base, self.limit)
    }
}

impl fmt::Display for kvm_segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{:04x}      {:016x}  {:08x}  {:02x}    {:x} {:x}   {:x}  {:x} {:x} {:x} {:x}",
            self.selector, self.base, self.limit, self.type_, self.present, self.dpl, self.db, self.s, self.l, self.g, self.avl)
    }
}
