// I think this is exhaustive of what I will actually get, but just in case...
#[non_exhaustive]
pub enum Irq {
    /// Programmable Interrupt Timer Interrupt
    Pit,
    /// Keyboard interrupts yay
    Keyboard,
    /// Never triggered, used internally by PICs
    Cascade,
    /// If enabled
    Com2,
    /// If enabled
    Com1,
    /// If enabled
    Lpt2,
    /// Floppy disk
    Floppy,
    /// Spurious interrupt
    Lpt1,
    CmosClock,
    /// Any of these
    FreeScsiNic1,
    /// Any of these
    FreeScsiNic2,
    /// Any of these
    FreeScsiNic3,
    /// Not the PlayStation 2, sorry
    Ps2Mouse,
    /// Any of these
    FpuCoprocessorInterprocessor,
    /// Primary
    AtaHard1,
    /// Secondary
    AtaHard2,
}
