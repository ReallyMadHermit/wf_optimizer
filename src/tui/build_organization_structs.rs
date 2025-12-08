use crate::mod_parsing::LoadedMods;

const ARCANE_RESULTS_COUNT: usize = 8;
const ARC: usize = ARCANE_RESULTS_COUNT;


pub struct BuildShowcase {
    top_builds: Vec<BuildSorter>,  // top build BuildDamage references refer to index in all_builds
    all_builds: Vec<[BuildSorter; ARC]>,  // references here refer to combo id
    pub len: usize
} impl BuildShowcase {

    pub fn from_manager(bucket_manager: &BucketManager) -> Self {
        let cap = bucket_manager.build_buckets.len();
        let mut top_builds = Vec::with_capacity(cap);
        let mut all_builds = Vec::with_capacity(cap);
        for (_, bucket) in &bucket_manager.build_buckets {
            let array = bucket.get_list(true);
            let mut top = array[0];
            top.reference = top_builds.len() as u32;
            top_builds.push(top);
            all_builds.push(array);
        }
        top_builds.sort_by_key(|build| build.inverse_damage);
        let len = top_builds.len();
        Self { top_builds, all_builds, len }
    }

    pub fn get_top_builds(&self) -> &[BuildSorter] {
        &self.top_builds
    }

    pub fn get_build_list(&self, arcane_id: usize) -> &[BuildSorter] {
        if arcane_id >= self.all_builds.len() {
            &[]
        } else {
            &self.all_builds[arcane_id]
        }
    }

    pub fn print_top_builds(&self, loaded_mods: &LoadedMods) {
        let arcanes = loaded_mods.get_arcane_names();
        for build in &self.top_builds{
            let arcane_name = if build.reference == 0 {
                "No arcane"
            } else {
                arcanes[build.get_reference()-1]
            };
            let build_damage = build.get_damage();
            println!("{}: {}", arcane_name, build_damage);
        }
    }

    pub fn print_all_builds(&self, loaded_mods: &LoadedMods) {
        let arcanes = loaded_mods.get_arcane_names();
        for (index, list) in self.all_builds.iter().enumerate() {
            if index == 0 {
                println!("No Arcane");
            } else {
                println!("{}", arcanes[index-1]);
            }
            for build in list {
                println!("{}", build.get_damage())
            }
        }
    }

}


pub struct BucketManager {
    build_buckets: Vec<(f32, BuildBucket)>
} impl BucketManager {

    pub fn new(arcane_count: usize) -> Self {
        Self {
            build_buckets: vec![(0.0, BuildBucket::default()); arcane_count+1]
        }
    }

    pub fn add(&mut self, build_damage: f32, build_combo_id: usize, arcane_id: usize) {
        if build_damage > self.build_buckets[arcane_id].0 {
            let new_damage = self.build_buckets[arcane_id].1.add(build_damage, build_combo_id);
            self.build_buckets[arcane_id].0 = new_damage;
        }
    }

}


#[derive(Clone)]
struct BuildBucket {
    damage_array: [f32; ARC],
    reference_array: [u32; ARC],
} impl BuildBucket {

    fn default() -> Self {
        Self {
            damage_array: [0.0; ARC],
            reference_array: [0; ARC]
        }
    }

    fn add(&mut self, new_damage: f32, new_reference: usize) -> f32 {
        let mut min_index = 0;
        let mut min_damage = self.damage_array[min_index];
        for i in 1..8 {
            if self.damage_array[i] < min_damage {
                min_damage = self.damage_array[i];
                min_index = i;
            }
        }
        if new_damage > min_damage {
            self.damage_array[min_index] = new_damage;
            self.reference_array[min_index] = new_reference as u32;
        }
        min_damage
    }

    fn get_list(&self, sort: bool) -> [BuildSorter; ARC] {
        let mut list = [BuildSorter::default(); ARC];
        for i in 0..ARC {
            list[i] = BuildSorter::new(self.damage_array[i], self.reference_array[i]);
        }
        if sort {
            list.sort_by_key(|build| build.inverse_damage);
        }
        list
    }

}


#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct BuildSorter {
    inverse_damage: u32,
    reference: u32
} impl BuildSorter {

    fn new(damage: f32, reference: u32) -> Self {
        Self {
            inverse_damage: u32::MAX - damage.round() as u32,
            reference
        }
    }

    fn default() -> Self {
        Self {
            inverse_damage: 0,
            reference: 0
        }
    }

    pub fn get_damage(&self) -> u32 {
        u32::MAX - self.inverse_damage
    }

    pub fn get_reference(&self) -> usize {
        self.reference as usize
    }

}