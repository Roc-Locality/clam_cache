use lease_cache_sim::LeaseCache;
use std::collections::HashMap;
use lease_cache_sim::TaggedObjectId;
use abstract_cache::ObjIdTraits;
use rand::Rng;

pub struct ClamCache <Ref: ObjIdTraits, Obj: ObjIdTraits> {
    pub cache: LeaseCache<Obj>,
    pub lease_table: HashMap<Ref, (usize, usize, f64)>
}

impl <Ref: ObjIdTraits, Obj: ObjIdTraits> ClamCache<Ref, Obj> {
    pub fn new(lease_table: HashMap<Ref, (usize, usize, f64)>, cache_size: usize) -> Self {
        let cache = LeaseCache::new();
        ClamCache {
            cache,
            lease_table
        }
    }

    pub fn sample_lease(&mut self, reference: Ref) -> usize {
        let (short_lease, long_lease, short_lease_prob) =
            *self.lease_table.get(&reference).unwrap();
        let mut rng = rand::thread_rng();
        let rand_num: f64 = rng.gen();
        let lease = if rand_num < short_lease_prob {
            short_lease
        } else {
            long_lease
        };
        return lease;
    }

}