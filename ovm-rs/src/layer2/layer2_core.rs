use plasma_db::impls::kvs::CoreDbMemoryImpl;
use plasma_db::traits::kvs::{BaseDbKey, Bucket, KeyValueStore};
use plasma_db::traits::db::DatabaseTrait;

pub struct Layer2Core {
    db: CoreDbMemoryImpl,
}

impl Default for Layer2Core {
    fn default() -> Self {
        Layer2Core {
            db: CoreDbMemoryImpl::open("test"),
        }
    }
}

impl Layer2Core {
    pub fn bucket(&self, prefix: &str) -> Bucket {
        self.db.bucket(&BaseDbKey::from(prefix.as_bytes()))
    }
}
