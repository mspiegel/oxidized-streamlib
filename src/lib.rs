
use std::collections::HashMap;

pub struct TopK<'lifetime>
{
    locations: HashMap<&'lifetime str, usize>,
    keys: Vec<&'lifetime str>,
    estimates: Vec<u64>,
    errors: Vec<u64>,
    capacity: usize
}

impl <'lifetime>TopK<'lifetime>
{

    pub fn new(capacity: usize) -> TopK<'lifetime>
    {
        TopK { locations: HashMap::new(), keys: Vec::new(), 
               estimates: Vec::new(), errors: Vec::new(), 
               capacity: capacity }
    }

    pub fn increment(&mut self, key: &'lifetime str)
    {
        self.update(key, 1)
    }

    pub fn update(&mut self, key: &'lifetime str, count: u64)
    {
        let mut located = false;
        let mut index: usize = 0; 
        {
            let entry = self.locations.get(key);
            if let Some(location) = entry {
                located = true;
                index = *location;
            }
        }
        if located {
            self.estimates[index] += count;
        } else if self.locations.len() < self.capacity {
            index = self.locations.len();
            self.locations.insert(key, index);
            self.keys.push(key);
            self.estimates.push(count);
            self.errors.push(0);
        } else {
            // TODO select random index when min estimates are tied
            index = self.capacity - 1;
            self.locations.remove(self.keys[index]);
            self.locations.insert(key, index);
            self.errors[index] = self.estimates[index];
            self.estimates[index] += 1;
        }
    }

    pub fn capacity(& self) -> usize { self.capacity }

    pub fn estimate(& self, key: &str) -> Option<u64>
    {
         match self.locations.get(key) {
             Some(location) => Some(self.estimates[*location]),
             None => None
         }
    }

    pub fn error(& self, key: &str) -> Option<u64>
    {
         match self.locations.get(key) {
             Some(location) => Some(self.errors[*location]),
             None => None
         }
    }

    pub fn estimate_and_error(& self, key: &str) -> Option<(u64, u64)>
    {
         match self.locations.get(key) {
             Some(location) => Some((self.estimates[*location], self.errors[*location])),
             None => None
         }
    }

}