use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x40024000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x800;
/// 0x40024000..0x40024800
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// AES Authentication Length
///
/// [TI-TRM-I] 10.9.1.22 AESAUTHLEN Register (Offset = 55Ch) [reset = 0h]
pub mod AESAUTHLEN;
/// AES Input/Output Buffer Control
///
/// [TI-TRM-I] 10.9.1.19 AESCTL Register (Offset = 550h) [reset = 80000000h]
pub mod AESCTL;
/// AES Data Input/Output 0
///
/// [TI-TRM-I] 10.9.1.24 AESDATAIN0 Register (Offset = 560h) [reset = 0h]
pub mod AESDATAIN0;
/// AES Data Input/Output 1
///
/// [TI-TRM-I] 10.9.1.26 AESDATAIN1 Register (Offset = 564h) [reset = 0h]
pub mod AESDATAIN1;
/// AES Data Input/Output 2
///
/// [TI-TRM-I] 10.9.1.28 AESDATAIN2 Register (Offset = 568h) [reset = 0h]
pub mod AESDATAIN2;
/// Data Input/Output
///
/// [TI-TRM-I] 10.9.1.30 AESDATAIN3 Register (Offset = 56Ch) [reset = 0h]
pub mod AESDATAIN3;
/// Crypto Data Length LSW
///
/// [TI-TRM-I] 10.9.1.20 AESDATALEN0 Register (Offset = 554h) [reset = 0h]
pub mod AESDATALEN0;
/// Crypto Data Length MSW
///
/// [TI-TRM-I] 10.9.1.21 AESDATALEN1 Register (Offset = 558h) [reset = 0h]
pub mod AESDATALEN1;
/// Data Input/Output
///
/// [TI-TRM-I] 10.9.1.23 AESDATAOUT0 Register (Offset = 560h) [reset = 0h]
pub mod AESDATAOUT0;
/// AES Data Input/Output 3
///
/// [TI-TRM-I] 10.9.1.25 AESDATAOUT1 Register (Offset = 564h) [reset = 0h]
pub mod AESDATAOUT1;
/// AES Data Input/Output 2
///
/// [TI-TRM-I] 10.9.1.27 AESDATAOUT2 Register (Offset = 568h) [reset = 0h]
pub mod AESDATAOUT2;
/// AES Data Input/Output 3
///
/// [TI-TRM-I] 10.9.1.29 AESDATAOUT3 Register (Offset = 56Ch) [reset = 0h]
pub mod AESDATAOUT3;
/// AES Initialization Vector
///
/// [TI-TRM-I] 10.9.1.18 AESIV_y Register (Offset = 540h + formula) [reset = 0h]
pub mod AESIV;
/// Clear AES_KEY2/GHASH Key
///
/// [TI-TRM-I] 10.9.1.16 AESKEY2_y Register (Offset = 500h + formula) [reset = 0h]
pub mod AESKEY2;
/// Clear AES_KEY3
///
/// [TI-TRM-I] 10.9.1.17 AESKEY3_y Register (Offset = 510h + formula) [reset = 0h]
pub mod AESKEY3;
/// AES Tag Output
///
/// [TI-TRM-I] 10.9.1.31 AESTAGOUT_y Register (Offset = 570h + formula) [reset = 0h]
pub mod AESTAGOUT;
/// Master Algorithm Select
///
/// This register configures the internal destination of the DMA controller.
///
/// [TI-TRM-I] 10.9.1.32 ALGSEL Register (Offset = 700h) [reset = 0h]
pub mod ALGSEL;
/// DMA Controller Master Configuration
///
/// [TI-TRM-I] 10.9.1.9 DMABUSCFG Register (Offset = 78h) [reset = 2400h]
pub mod DMABUSCFG;
/// DMA Channel 0 Control
///
/// [TI-TRM-I] 10.9.1.1 DMACH0CTL Register (Offset = 0h) [reset = 0h]
pub mod DMACH0CTL;
/// DMA Channel 0 External Address
///
/// [TI-TRM-I] 10.9.1.2 DMACH0EXTADDR Register (Offset = 4h) [reset = 0h]
pub mod DMACH0EXTADDR;
/// DMA Channel 0 Length
///
/// [TI-TRM-I] 10.9.1.3 DMACH0LEN Register (Offset = Ch) [reset = 0h]
pub mod DMACH0LEN;
/// DMA Channel 1 Control
///
/// [TI-TRM-I] 10.9.1.6 DMACH1CTL Register (Offset = 20h) [reset = 0h]
pub mod DMACH1CTL;
/// DMA Channel 1 External Address
///
/// [TI-TRM-I] 10.9.1.7 DMACH1EXTADDR Register (Offset = 24h) [reset = 0h]
pub mod DMACH1EXTADDR;
/// DMA Channel 1 Length
///
/// [TI-TRM-I] 10.9.1.8 DMACH1LEN Register (Offset = 2Ch) [reset = 0h]
pub mod DMACH1LEN;
/// DMA Controller Version
///
/// [TI-TRM-I] 10.9.1.11 DMAHWVER Register (Offset = FCh) [reset = 01012ED1h]
pub mod DMAHWVER;
/// DMA Controller Port Error
///
/// [TI-TRM-I] 10.9.1.10 DMAPORTERR Register (Offset = 7Ch) [reset = 0h]
pub mod DMAPORTERR;
/// Master Protection Control
///
/// [TI-TRM-I] 10.9.1.33 DMAPROTCTL Register (Offset = 704h) [reset = 0h]
pub mod DMAPROTCTL;
/// DMA Controller Status
///
/// [TI-TRM-I] 10.9.1.4 DMASTAT Register (Offset = 18h) [reset = 0h]
pub mod DMASTAT;
/// DMA Controller Software Reset
///
/// [TI-TRM-I] 10.9.1.5 DMASWRESET Register (Offset = 1Ch) [reset = 0h]
pub mod DMASWRESET;
/// CTRL Module Version
///
/// [TI-TRM-I] 10.9.1.40 HWVER Register (Offset = 7FCh) [reset = 91118778h]
pub mod HWVER;
/// Interrupt Clear
///
/// [TI-TRM-I] 10.9.1.37 IRQCLR Register (Offset = 788h) [reset = 0h]
pub mod IRQCLR;
/// Interrupt Enable
///
/// [TI-TRM-I] 10.9.1.36 IRQEN Register (Offset = 784h) [reset = 0h]
pub mod IRQEN;
/// Interrupt Set
///
/// [TI-TRM-I] 10.9.1.38 IRQSET Register (Offset = 78Ch) [reset = 0h]
pub mod IRQSET;
/// Interrupt Status
///
/// [TI-TRM-I] 10.9.1.39 IRQSTAT Register (Offset = 790h) [reset = 0h]
pub mod IRQSTAT;
/// Control Interrupt Configuration
///
/// [TI-TRM-I] 10.9.1.35 IRQTYPE Register (Offset = 780h) [reset = 0h]
pub mod IRQTYPE;
/// Key Read Area
///
/// [TI-TRM-I] 10.9.1.15 KEYREADAREA Register (Offset = 40Ch) [reset = 8h]
pub mod KEYREADAREA;
/// Key Size
///
/// This register defines the size of the keys that are written with DMA.
///
/// [TI-TRM-I] 10.9.1.14 KEYSIZE Register (Offset = 408h) [reset = 1h]
pub mod KEYSIZE;
/// Key Write Area
///
/// [TI-TRM-I] 10.9.1.12 KEYWRITEAREA Register (Offset = 400h) [reset = 0h]
pub mod KEYWRITEAREA;
/// Key Written Area Status
///
/// This register shows which areas of the key store RAM contain valid written keys.
///
/// When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory.
///
/// Attempting to write to a key area that already contains a valid key is not allowed and will result in an error.
///
/// [TI-TRM-I] 10.9.1.13 KEYWRITTENAREA Register (Offset = 404h) [reset = 0h]
pub mod KEYWRITTENAREA;
/// Software Reset
///
/// [TI-TRM-I] 10.9.1.34 SWRESET Register (Offset = 740h) [reset = 0h]
pub mod SWRESET;
