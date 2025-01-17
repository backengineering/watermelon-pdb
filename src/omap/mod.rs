// Copyright (C) Back Engineering Labs, Inc. - All Rights Reserved

/// Maps from new addresses to original ones in the binary. If it doesn't map
/// to anything it is [`Self::INVALID_ADDRESS`].
pub struct MlnOMap {
    pub map: Vec<u32>,
}
impl MlnOMap {
    pub const OMAP_INVALID_ADDRESS: u32 = 0xFFFFFFFF;
}
