//! Simulation core: Time tracking, deterministic PRNG, and event recording.

/// Tracking simulation ticks and in-game day progression.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationTime {
    /// Monotonically increasing day counter.
    pub day: u32,
    /// Hour of the current day (0..23).
    pub hour: u8,
    /// Total ticks executed since simulation start.
    pub total_ticks: u64,
}

impl SimulationTime {
    pub fn new() -> Self {
        Self {
            day: 1,
            hour: 0,
            total_ticks: 0,
        }
    }

    /// Advance time by one simulation tick (representing 1 game hour).
    /// Returns true if a new 24-hour daily economic/demographic update occurred.
    pub fn advance_hour(&mut self) -> bool {
        self.total_ticks += 1;
        self.hour += 1;
        if self.hour >= 24 {
            self.hour = 0;
            self.day += 1;
            true
        } else {
            false
        }
    }

    /// Advance time by a full 24-hour cycle directly.
    pub fn advance_day(&mut self) {
        self.total_ticks += 24;
        self.day += 1;
        self.hour = 0;
    }
}

impl Default for SimulationTime {
    fn default() -> Self {
        Self::new()
    }
}

/// Fast, deterministic 64-bit XorShift Pseudo-Random Number Generator.
///
/// Ensures 100% deterministic simulation outcomes across platforms and runs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    /// Initialize with a non-zero seed.
    pub fn new(seed: u64) -> Self {
        let state = if seed == 0 { 0x853c49e6748fea9b } else { seed };
        Self { state }
    }

    /// Expose internal state for deterministic persistence.
    pub fn state(&self) -> u64 {
        self.state
    }

    /// Restore exact internal state from persistence.
    pub fn from_state(state: u64) -> Self {
        let s = if state == 0 { 0x853c49e6748fea9b } else { state };
        Self { state: s }
    }

    /// Generate the next raw pseudo-random u64.
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    /// Generate a random integer in the range `[min, max]` inclusive.
    pub fn gen_range_i32(&mut self, min: i32, max: i32) -> i32 {
        if min >= max {
            return min;
        }
        let range = (max - min + 1) as u64;
        let val = (self.next_u64() % range) as i32;
        min + val
    }

    /// Generate a random percentage roll in `[0, 100)`.
    pub fn gen_percent(&mut self) -> u32 {
        (self.next_u64() % 100) as u32
    }
}

/// Simulation event emitted during execution ticks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimulationEvent {
    SimulationStarted { day: u32 },
    DayAdvanced { day: u32 },
    HoldingCaptured { holding_id: u8, new_owner: &'static str },
    TaxesCollected { total_gold: i64 },
    FoodDistributed { total_consumed: i32, net_surplus: i32 },
    PopulationGrown { holding_id: u8, increase: i32 },
    TroopsRecruited { holding_id: u8, count: i32 },
    TreatySigned { other_faction: &'static str, duration_days: u32 },
    UnrestTriggered { holding_id: u8, loyalty: i32 },
}

/// Standard simulation error enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimulationError {
    InvalidHoldingId(u8),
    InvalidDivisionId(u8),
    InsufficientFunds { required: i64, available: i64 },
    InsufficientResources { resource: &'static str, required: i32, available: i32 },
    InvalidStateTransition(&'static str),
    HoldingNotAdjacent { from: u8, to: u8 },
}

pub type Result<T> = std::result::Result<T, SimulationError>;
