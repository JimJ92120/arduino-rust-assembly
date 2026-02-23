mod port_b;
mod port_d;
mod port_c;

pub use port_b::PortB;
pub use port_d::PortD;
pub use port_c::PortC;

pub trait Port {
    const PORT_ADDRESS: *mut u8;
    const DDR_ADDRESS: *mut u8;
}
