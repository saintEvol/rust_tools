use std::sync::Arc;
use std::time::SystemTimeError;
use parking_lot::RwLock;
use uuid::{Timestamp, Uuid};

#[derive(Clone)]
pub struct SharedUuidV7Generator(Arc<RwLock<UuidV7Generator>>);
impl SharedUuidV7Generator {
    pub fn from_generator(generator: UuidV7Generator) -> Self {
        Self(Arc::new(RwLock::new(generator)))
    }

    pub fn now_uuid(&self) -> Result<Uuid, SystemTimeError> {
        let mut inner = self.0.write();
        inner.now_uuid()
    }
}

/// [node_id]节点id，用于分片
/// [counter_bits]表示计数器大小，推荐
/// [node_id]，[counter]部分会共同组成除[时间]部分的剩余部分(62位)，如果还有剩余空间，由会由随机数填充
/// 总体组成:[时间戳(48 bits)][version (4bits)][variant(2 bits)][combined (node_id(16 bits) + counter(counter_bits))] [random]
pub struct UuidV7Generator {
    node_id: u16,
    counter: u32,
    counter_bits: u8,
}

impl UuidV7Generator {
    pub fn with_default_counter_bits(node_id: u16) -> Self {
        Self::new(node_id, 24)
    }

    pub fn new(node_id: u16, counter_bits: u8) -> Self {
        Self {
            node_id: node_id,
            counter: 0,
            counter_bits: counter_bits,
        }
    }

    pub fn now_uuid(&mut self) -> Result<Uuid, SystemTimeError> {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?;
        let sec = now.as_secs();
        let nanos = now.subsec_nanos();
        // node id + 计数器
        let combined = ((self.node_id as u128) << self.counter_bits) | (self.counter as u128);
        self.counter = self.counter.wrapping_add(1);
        let ts = Timestamp::from_unix_time(
            sec,
            nanos,
            combined,
            self.counter_bits + 16, /* 计数部分大小：节点（16位）+ counter_bits */
        );
        Ok(Uuid::new_v7(ts))
    }

    // pub fn ms(uuid: &Uuid) -> u64 {
    //     let Timestamp {
    //         seconds,
    //         subsec_nanos,
    //         ..
    //     } = uuid.get_timestamp();

    // }
}
