use std::collections::BTreeMap;

pub struct IntensitySegments {
    map: BTreeMap<i128, i128>
}

impl IntensitySegments {
    pub fn new() -> Self {
        IntensitySegments {map: BTreeMap::new()}
    }
    
    pub fn add(&mut self, from: i128, to: i128, amount: i128){
        if amount == 0 {
            return;
        }

        if let Some((k, v)) = self.map.range(..=to).next_back(){
            // keep 0 -> 0    5 -> 5
            self.map.insert(to, *v);
        } else {
            // border
            self.map.insert(to, 0);
        }

        if let Some((k, v)) = self.map.range(..=from).next_back(){
            // handle == front or == back
            self.map.insert(to, *v + amount);
        } else {
            // new value, border
            self.map.insert(to, amount);
        }

        let keys_to_add: Vec<i128> = self.map
            .range((from + 1)..to)
            .map(|(k, _)| *k)
            .collect();

        for key in keys_to_add {
            self.map.insert(key, self.map.get(&key).unwrap() + amount                                                                          );
        }
    }

    pub fn set(&mut self, from: i128, to: i128, amount: i128){
        if amount == 0 {
            if self.map.range((to + 1)..).next().is_some() {
                if let Some((k, v)) = self.map.range(..=to).next_back() {
                    self.map.insert(to, *v);
                } else {
                    // no need to insert
                }
            } else {
                // no need to insert
            }

            if self.map.range((from + 1)..).next().is_some() {
                if let Some((k, v)) = self.map.range(..from).next_back() {
                    if *v ==0 {
                        // no need to change
                    } else {
                        self.map.insert(from, 0);
                    }
                } else {
                    // no need to insert
                }
            } else {
                // no need to insert
            }
        } else {
            if let Some((k, v)) = self.map.range(..=to).next_back() {
                self.map.insert(to, *v);
            } else {
                self.map.insert(to, 0);
            }

            self.map.insert(from, amount);

            let keys_to_remove: Vec<i128> = self.map
                .range((from + 1)..to)
                .map(|(k, _)| *k)
                .collect();

            for key in keys_to_remove {
                self.map.remove(&key);
            }
        }
    }

    pub fn to_string(&self) -> String{
        format!("[{}]", self
            .map
            .iter()
            .map(|(k, v)| format!("[{},{}]", *k, *v))
            .collect::<Vec<String>>()
            .join(",")
        )
    }
}