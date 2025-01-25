use std::{
    env, fs::File, io::Read, path::PathBuf, slice::from_raw_parts, thread::sleep, time::Duration,
};

use kvm_bindings::*;
use kvm_ioctls::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the guest binary
    // Get the output directory from the environment variable
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let binary_file_path = PathBuf::from(out_dir).join("guest");

    // Open the binary file
    let mut file = File::open(&binary_file_path)?;

    let mut buffer = Vec::new();
    file.read_to_end(buffer.as_mut())?;

    // Create VM
    let vm = Kvm::new()?;
    let vm_fd: VmFd = vm.create_vm()?;

    // Assign Memory
    // From: https://github.com/virtee/sev/blob/18ed5c50eb0bd71b92c68bd96a22ab589a76ab21/tests/sev_launch.rs#L57
    let mem_size = 0x1000;
    let userspace_addr = unsafe { libc::mmap(0 as _, mem_size, 3, 34, -1, 0) };
    if userspace_addr == libc::MAP_FAILED {
        panic!("mmap() failed");
    }

    unsafe {
        std::ptr::copy_nonoverlapping(buffer.as_ptr(), userspace_addr as *mut u8, buffer.len())
    };

    let userspace_addr: &[u8] = unsafe { from_raw_parts(userspace_addr as *mut u8, mem_size) };

    let user_memory_region: kvm_userspace_memory_region = kvm_userspace_memory_region {
        slot: 0,
        guest_phys_addr: 0,
        memory_size: mem_size as _,
        userspace_addr: userspace_addr.as_ptr() as _,
        flags: 0,
    };

    unsafe {
        vm_fd.set_user_memory_region(user_memory_region)?;
    }

    // Create the VCPU
    let mut vcpu_fd = vm_fd.create_vcpu(0)?;
    let mut sregs = vcpu_fd.get_sregs()?;
    // Set the selector and base sregs to 0
    sregs.cs.selector = 0;
    sregs.cs.base = 0;
    sregs.ss.selector = 0;
    sregs.ds.selector = 0;
    sregs.ds.base = 0;
    sregs.es.selector = 0;
    sregs.es.base = 0;
    sregs.fs.selector = 0;
    sregs.fs.base = 0;
    sregs.gs.selector = 0;
    sregs.gs.base = 0;
    vcpu_fd.set_sregs(&sregs)?;

    let mut regs = vcpu_fd.get_regs()?;
    regs.rip = 0;
    regs.rflags = 2;
    vcpu_fd.set_regs(&regs)?;

    loop {
        match vcpu_fd.run()? {
            VcpuExit::Shutdown => {
                println!("Shutdown");
                break;
            }
            VcpuExit::IoOut(addr, data) => {
                println!("IOOut at {:#x} data: {:#x?}", addr, data);
            }
            r => {
                println!("Unknown exit reason: {:?}", r);
                break;
            }
        }
        sleep(Duration::from_secs(1));
    }

    drop(vcpu_fd);
    drop(vm_fd);

    Ok(())
}
