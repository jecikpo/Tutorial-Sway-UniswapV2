library;

pub struct MintEvent {
    /// Identifies the address who originated the call.
    pub sender: Identity,
    /// Identifies the address whom the LP token is mint to.
    pub to: Identity,
    /// Provided amount of token0
    pub amount0: u64,
    /// Provided amount of token1
    pub amount1: u64,
}

pub struct BurnEvent {
    /// Identifies the address who originated the call.
    pub sender: Identity,
    /// Identifies the address whom the LP token is mint to.
    pub to: Identity,
    /// Provided amount of token0
    pub amount0: u64,
    /// Provided amount of token1
    pub amount1: u64,
}

pub struct SwapEvent {
    /// Identifies the address who originated the call.
    pub sender: Identity,
    /// Identifies the address where the out tokens are sent to.
    pub to: Identity,
    /// Provided amount of token0 in
    pub amount0_in: u64,
    /// Provided amount of token1 in
    pub amount1_in: u64,
    /// Amount of token0 sent out
    pub amount0_out: u64,
    /// Amount of token0 sent out
    pub amount1_out: u64,
}