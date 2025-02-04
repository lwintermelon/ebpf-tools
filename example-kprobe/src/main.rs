#![cfg_attr(feature = "kern", no_std, no_main, feature(lang_items))]

#[cfg(feature = "kern")]
use ebpf_kern as ebpf;
#[cfg(feature = "user")]
use ebpf_user as ebpf;

#[cfg(feature = "kern")]
ebpf::license!("Dual BSD/GPL");

#[cfg(any(feature = "kern", feature = "user"))]
#[derive(ebpf::BpfApp)]
pub struct App {
    #[ringbuf(size = 0x10000)]
    pub event_queue: ebpf::RingBufferRef,
    #[prog("tracepoint/exceptions/page_fault_user")]
    pub page_fault: ebpf::ProgRef,
}

#[cfg(feature = "kern")]
impl App {
    #[inline(always)]
    pub fn page_fault(&mut self, ctx: ebpf::Context) -> Result<(), i32> {
        let addr = ctx.read::<u64>(0);

        self.event_queue.output(&addr.to_ne_bytes())
    }
}

#[cfg(feature = "user")]
fn main() -> Result<(), i32> {
    use std::{time::Duration, convert::TryFrom};
    use ebpf::{Skeleton, RingBufferRegistry};

    static CODE: &[u8] = include_bytes!("../../target/bpfel-unknown-none/release/example-kern");
    let mut skeleton = Skeleton::<App>::open("example-kprobe\0", CODE)?;
    skeleton.load()?;
    skeleton.attach()?;

    let mut rb = RingBufferRegistry::default();
    rb.add(&skeleton.app.event_queue, |s| {
        println!("{:x}", u64::from_ne_bytes(TryFrom::try_from(s).unwrap()));
    })?;

    loop {
        match rb.poll(Duration::from_millis(100)) {
            Ok(_) => (),
            Err(c) if c == -4 => break Ok(()),
            Err(c) => break Err(c),
        }
    }
}
