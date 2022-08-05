#[derive(Clone, Copy, Debug, Default, Hash, Ord, PartialEq, PartialOrd, Eq)]
pub struct ActorId([u8; 32]);

impl ActorId {
    pub const fn new(arr: [u8; 32]) -> Self {
        Self(arr)
    }

    pub const fn zero() -> Self {
        Self::new([0; 32])
    }
}

impl AsRef<[u8]> for ActorId {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsMut<[u8]> for ActorId {
    fn as_mut(&mut self) -> &mut [u8] {
        self.0.as_mut()
    }
}

impl From<u64> for ActorId {
    fn from(v: u64) -> Self {
        let mut arr = [0u8; 32];
        arr[0..8].copy_from_slice(&v.to_le_bytes()[..]);
        Self(arr)
    }
}

impl From<[u8; 32]> for ActorId {
    fn from(arr: [u8; 32]) -> Self {
        Self(arr)
    }
}

impl From<ActorId> for [u8; 32] {
    fn from(other: ActorId) -> Self {
        other.0
    }
}
