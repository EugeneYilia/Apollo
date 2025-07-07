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
            self.map.insert(from, *v + amount);
        } else {
            // new value, border
            self.map.insert(from, amount);
        }

        let keys_to_add: Vec<i128> = self.map
            .range((from + 1)..to)
            .map(|(k, _)| *k)
            .collect();

        for key in keys_to_add {
            self.map.insert(key, self.map.get(&key).unwrap() + amount                                                                          );
        }
        
        if let Some((k, v)) = self.map.range(..from).next_back(){
            if *v == *self.map.get(&from).unwrap() {
                self.map.remove(&from);
            }
        } else {
            if 0 == *self.map.get(&from).unwrap() {
                self.map.remove(&from);
            }
        }

        if let Some((k, v)) = self.map.range(..to).next_back(){
            if *v == *self.map.get(&to).unwrap() {
                self.map.remove(&to);
            } else {
                if 0 == *self.map.get(&to).unwrap() {
                    self.map.remove(&to);
                }
            }
        }
    }

    pub fn set(&mut self, from: i128, to: i128, amount: i128){
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

        if let Some((k, v)) = self.map.range(..from).next_back(){
            if *v == *self.map.get(&from).unwrap() {
                self.map.remove(&from);
            }
        } else {
            if 0 == *self.map.get(&from).unwrap() {
                self.map.remove(&from);
            }
        }

        if let Some((k, v)) = self.map.range(..to).next_back(){
            if *v == *self.map.get(&to).unwrap() {
                self.map.remove(&to);
            } else {
                if 0 == *self.map.get(&to).unwrap() {
                    self.map.remove(&to);
                }
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