use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x400c5000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x1000;
/// 0x400c5000..0x400c6000
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Combined Event To MCU Mask
///
///
///
/// Select event flags in EVTOMCUFLAGS that contribute to the AUX_COMB event to EVENT and system CPU.
///
///
///
/// The AUX_COMB event is high as long as one or more of the included event flags are set.
///
/// [TI-TRM-I] 17.7.3.12 COMBEVTOMCUMASK Register (Offset = 2Ch) [reset = 0h]
pub mod COMBEVTOMCUMASK;
/// Direct Memory Access Control
///
/// [TI-TRM-I] 17.7.3.6 DMACTL Register (Offset = 14h) [reset = 0h]
pub mod DMACTL;
/// Event Status 0
///
///
///
/// Register holds events 0 thru 15 of the 32-bit event bus that is synchronous to AUX clock. The following subscribers use the asynchronous version of events in this register.
///
/// - AUX_ANAIF.
///
/// - AUX_TDC.
///
/// [TI-TRM-I] 17.7.3.8 EVSTAT0 Register (Offset = 1Ch) [reset = 0h]
pub mod EVSTAT0;
/// Event Status 1
///
///
///
/// Current event source levels, 31:16
///
/// [TI-TRM-I] 17.7.3.9 EVSTAT1 Register (Offset = 20h) [reset = 0h]
pub mod EVSTAT1;
/// Events To AON Flags
///
///
///
/// This register contains a collection of event flags routed to AON_EVENT.
///
///
///
/// To clear an event flag, write to EVTOAONFLAGSCLR or write 0 to event flag in this register.
///
/// [TI-TRM-I] 17.7.3.4 EVTOAONFLAGS Register (Offset = Ch) [reset = 0h]
pub mod EVTOAONFLAGS;
/// Events To AON Clear
///
///
///
/// Clear event flags in EVTOAONFLAGS.
///
///
///
/// In order to clear a level sensitive event flag, the event must be deasserted.
///
/// [TI-TRM-I] 17.7.3.15 EVTOAONFLAGSCLR Register (Offset = 3Ch) [reset = 0h]
pub mod EVTOAONFLAGSCLR;
/// Events To AON Polarity
///
///
///
/// Event source polarity configuration for EVTOAONFLAGS.
///
/// [TI-TRM-I] 17.7.3.5 EVTOAONPOL Register (Offset = 10h) [reset = 0h]
pub mod EVTOAONPOL;
/// Events to MCU Flags
///
///
///
/// This register contains a collection of event flags routed to MCU domain.
///
///
///
/// To clear an event flag, write to EVTOMCUFLAGSCLR or write 0 to event flag in this register. Follow procedure described in AUX_SYSIF:WUCLR to clear AUX_WU_EV event flag.
///
/// [TI-TRM-I] 17.7.3.11 EVTOMCUFLAGS Register (Offset = 28h) [reset = 0h]
pub mod EVTOMCUFLAGS;
/// Events To MCU Flags Clear
///
///
///
/// Clear event flags in EVTOMCUFLAGS.
///
///
///
/// In order to clear a level sensitive event flag, the event must be deasserted.
///
/// [TI-TRM-I] 17.7.3.14 EVTOMCUFLAGSCLR Register (Offset = 38h) [reset = 0h]
pub mod EVTOMCUFLAGSCLR;
/// Event To MCU Polarity
///
///
///
/// Event source polarity configuration for EVTOMCUFLAGS.
///
/// [TI-TRM-I] 17.7.3.10 EVTOMCUPOL Register (Offset = 24h) [reset = 0h]
pub mod EVTOMCUPOL;
/// Sensor Controller Engine Wait Event Selection
///
///
///
/// Configuration of this register controls bit index 7 in AUX_SCE:WUSTAT.EV_SIGNALS. This bit can be used by AUX_SCE WEV0, WEV1, BEV0 and BEV1 instructions
///
/// [TI-TRM-I] 17.7.3.3 SCEWEVSEL Register (Offset = 8h) [reset = 0h]
pub mod SCEWEVSEL;
/// Software Event Set
///
///
///
/// Set software event flags from AUX domain to AON and MCU domains. CPUs in MCU domain can read the event flags from EVTOAONFLAGS and clear them in EVTOAONFLAGSCLR.
///
///
///
/// Use of these event flags is software-defined.
///
/// [TI-TRM-I] 17.7.3.7 SWEVSET Register (Offset = 18h) [reset = 0h]
pub mod SWEVSET;
/// Vector Configuration 0
///
///
///
/// AUX_SCE wakeup vector 0 and 1 configuration
///
/// [TI-TRM-I] 17.7.3.1 VECCFG0 Register (Offset = 0h) [reset = 0h]
pub mod VECCFG0;
/// Vector Configuration 1
///
///
///
/// AUX_SCE event vectors 2 and 3 configuration
///
/// [TI-TRM-I] 17.7.3.2 VECCFG1 Register (Offset = 4h) [reset = 0h]
pub mod VECCFG1;
/// Vector Flags
///
///
///
/// If a vector flag becomes 1 and AUX_SCE sleeps, AUX_SCE will wake up and execute the corresponding vector. The vector with the lowest index will execute first if multiple vectors flags are set. AUX_SCE must return to sleep to execute the next vector.
///
///
///
/// During execution of a vector, AUX_SCE must clear the vector flag that triggered execution. Write 1 to bit index n in VECFLAGSCLR to clear vector flag n.
///
/// [TI-TRM-I] 17.7.3.13 VECFLAGS Register (Offset = 34h) [reset = 0h]
pub mod VECFLAGS;
/// Vector Flags Clear
///
///
///
/// Strobes for clearing flags in VECFLAGS.
///
/// [TI-TRM-I] 17.7.3.16 VECFLAGSCLR Register (Offset = 40h) [reset = 0h]
pub mod VECFLAGSCLR;
