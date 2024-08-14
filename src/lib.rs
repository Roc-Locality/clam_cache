mod clam_cache_file_reader;

use abstract_cache::CacheSim;
use abstract_cache::ObjIdTraits;
use lease_cache_sim::LeaseCache;
use lease_cache_sim::TaggedObjectId;
use rand::Rng;
use std::collections::HashMap;

pub struct ClamCache<Ref: ObjIdTraits, Obj: ObjIdTraits> {
    pub cache: LeaseCache<Obj>,
    pub lease_table: HashMap<Ref, (usize, usize, f64)>,
}

impl<Ref: ObjIdTraits, Obj: ObjIdTraits> ClamCache<Ref, Obj> {
    pub fn new(lease_table: HashMap<Ref, (usize, usize, f64)>) -> Self {
        let cache = LeaseCache::new();
        ClamCache { cache, lease_table }
    }

    pub fn sample_lease(&mut self, reference: Ref) -> usize {
        let (short_lease, long_lease, short_lease_prob) =
            *self.lease_table.get(&reference).unwrap();
        let mut rng = rand::thread_rng();
        let rand_num: f64 = rng.gen();
        if rand_num < short_lease_prob {
            short_lease
        } else {
            long_lease
        }
    }
}

impl<Tag: ObjIdTraits, Obj: ObjIdTraits> CacheSim<TaggedObjectId<Tag, Obj>>
    for ClamCache<Tag, Obj>
{
    fn cache_access(&mut self, access: TaggedObjectId<Tag, Obj>) -> abstract_cache::AccessResult {
        let TaggedObjectId(tag, obj_id) = access;
        let lease = self.sample_lease(tag);
        self.cache.cache_access(TaggedObjectId(lease, obj_id))
    }

    fn set_capacity(&mut self, cache_size: usize) -> &mut Self {
        self.cache.set_capacity(cache_size);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clam_cache_file_reader;
    use std::collections::HashMap;

    #[test]
    fn test_lease_cache_tag_id() {
        let tag_id_iter = vec![
            //TaggedObjectId(Ref, ObjId)
            TaggedObjectId(1, 2), // lease 2
            TaggedObjectId(3, 4), //lease 1
            TaggedObjectId(1, 2), //lease 2
        ]
        .into_iter();
        let lease_map: HashMap<u64, (usize, usize, f64)> = vec![(1, (2, 0, 1.0)), (3, (1, 0, 1.0))]
            .into_iter()
            .collect();
        let mut clam_cache = ClamCache::<u64, u64>::new(lease_map);
        clam_cache.set_capacity(1000);
        let mr = clam_cache.get_mr(tag_id_iter);
        assert_eq!(mr, 1.0);
        println!("mr: {}", mr);
    }

    // #[test]
    // fn get_mr_for_3mm() {
    //     let lease_map = clam_cache_file_reader::lease_to_map("./src/polybench/3mm/3mm_output_shel_leases".to_string());
    //     let mut lease_cache = ClamCache::<u64, u64>::new(lease_map);
    //     lease_cache.set_capacity(1000);
    //     let trace = clam_cache_file_reader::trace_to_vec_u64("./src/polybench/3mm/3mm_output.txt".to_string());
    //     let mr = lease_cache.get_mr(trace.into_iter());
    //     println!("mr: {}", mr);
    //     assert!(true);
    // }

    #[test]
    fn test_sample_lease() {
        let num_iters = 1000;
        let lease_map: HashMap<usize, (usize, usize, f64)> =
            vec![(0, (0, 1, 0.5))].into_iter().collect();
        let mut num_short_lease = 0;
        let mut num_long_lease = 0;
        let mut clam_cache = ClamCache::<usize, usize>::new(lease_map);
        (0..num_iters).for_each(|_| {
            let lease = clam_cache.sample_lease(0);
            match lease {
                0 => num_short_lease += 1,
                1 => num_long_lease += 1,
                _ => panic!("Invalid lease"),
            }
        });
        assert!(
            (num_short_lease as f64 / num_iters as f64 - num_long_lease as f64 / num_iters as f64)
                < 0.02
        );
        println!(
            "short_lease_prob: {} long_lease_prob: {}, ",
            num_short_lease as f64 / num_iters as f64,
            num_long_lease as f64 / num_iters as f64
        );
    }
}
