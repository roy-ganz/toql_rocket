use std::sync::RwLock;
use std::ops::Deref;

struct Cache { 
    cache: RwLock<crate::toql::cache::Cache> 
    }

impl Cache {
    pub fn new() -> Self {
        Cache {
            cache: RwLock::new(crate::toql::cache::Cache::new())
        }
    }
}

impl Deref for Cache {

    fn deref(&self) -> &mut crate::toql::cache::Cache {
        self.cache
    }
}


