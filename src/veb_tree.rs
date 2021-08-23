#[derive(Clone,Debug, PartialEq, Eq)]
pub struct VEBTree<T> {
    children: Vec<VEBTree<T>>,
    aux: Vec<VEBTree<T>>,
    max: Option<T>,
    min: Option<T>

}

impl VEBTree<T> {
    pub fn new(max_size: usize) -> Self {
        let self_size: usize;
        let pass_size: usize;
        if max_size <= 2 {
            self_size = 0;
            pass_size = 0;
        } else {
            let tmp = (max_size as f64).sqrt().ceil() as usize;
            self_size = tmp;
            pass_size = tmp;
        }
        let mut children_seed: Vec<VEBTree> = Vec::with_capacity(self_size);
        let mut aux_seed: Vec<VEBTree> = Vec::with_capacity(1);
        if pass_size > 0 {
            for index in 0..self_size {
                children_seed.push(Self::new(pass_size));
            }
            let mut aux = Self::new(pass_size);
            aux_seed.push(aux);
        }
        let mut tree = VEBTree {
            children: children_seed,
            aux: aux_seed,
            max: None,
            min: None
        };
        return tree
    }
}

impl<T> VEBTree<T> {
    fn high(&self, value: T) -> T {
        return value / self.children.len();
    }

    fn low(&self, value: T) -> T {
        return value % self.children.len();
    }

    pub fn contains(&self, value: T) -> bool {
        match self.min {
            Some(min_val) => {
                if value == min_val {
                    return true;
                } else {
                    match self.max {
                        Some(max_val) => {
                            if value == max_val {
                                return true;
                            } else {
                                if self.children.len() > 0 {
                                    return self
                                        .children[self.high(value)]
                                        .contains(self.low(value));
                                } else {
                                    return false;
                                }
                            }
                        },
                        None => {return false;}
                    }
                }
            },
            None => {return false;}
        }
    }

    pub fn search(&self, value: T) -> Option<T> {
        let min_val = self.min?;
        if value == min_val {
            return self.min;
        }
        let max_val = self.max?;
        if value == max_val {
            return self.max;
        }
        if self.children.len() == 0 {
            return None;
        } else {
            let local_idx = self.high(value);
            let pass_value = self.low(value);
            self.children[local_idx].minimum()?;
            let result = self.children[local_idx].search(pass_value)?;
            return Some(result+local_idx*self.children.len());
        }
    }

    fn insert_into_tree(&mut self, value: T) {
        if self.children.len() > 0 {
            let local_idx = self.high(value);
            let pass_value = self.low(value);
            match self.children[local_idx].minimum() {
                Some(min_value) => (),
                None => {
                    self.aux[0].insert(local_idx);
                }
            };
            self.children[local_idx].insert(pass_value);
        }
    }

    pub fn insert(&mut self, value: T) {
        match self.min {
            Some(min_value) => {
                if value == min_value {
                    return;
                } else {
                    match self.max {
                        Some(max_value) => {
                            if max_value == value {
                                return;
                            } else {
                                if value < min_value {
                                    self.min = Some(value);
                                    self.insert_into_tree(value);
                                    return;
                                } else if value > max_value {
                                    self.max = Some(value);
                                    self.insert_into_tree(value);
                                    return;
                                } else {
                                    self.insert_into_tree(value);
                                    return;
                                }
                            }
                        },
                        None => {
                            self.max = Some(value);
                            self.insert_into_tree(value);
                            return;
                        }
                    };
                }
            },
            None => {
                self.min = Some(value);
                self.max = Some(value);
                self.insert_into_tree(value);
                return;
            }
        };

    }

    fn delete_from_tree(&mut self, value: T) {
        if self.children.len() > 0 {
            let local_idx = self.high(value);
            let pass_value = self.low(value);
            self.children[local_idx].delete(pass_value);
            if self.children[local_idx].minimum() == None {
                self.aux[0].delete(local_idx);
            }
        }
    }

    pub fn delete(&mut self, value: T){
        match self.min {
            Some(min_value) => {
                match self.max {
                    Some(max_value) => {
                        if max_value == min_value {
                            self.delete_from_tree(value);
                            self.min = None;
                            self.max = None;
                            return;
                        } else if self.children.len() == 0 {
                            if value == min_value {
                                self.min = Some(max_value);
                            } else {
                                self.max = Some(min_value);
                            }
                        } else if value == min_value {
                            self.delete_from_tree(value);
                            let first_populated = self.aux[0].minimum();
                            match first_populated {
                                Some(first_cluster) => {
                                    let new_min = self
                                        .children[first_cluster as usize]
                                        .minimum();
                                    match new_min {
                                        Some(min) => {
                                            self.min = Some((first_cluster
                                                * self.children.len() as T
                                                + min));
                                            return;
                                        },
                                        None => {
                                            panic!("Data structure appears corrupt");
                                        }
                                    };
                                },
                                None => {
                                    self.max = None;
                                    self.min = None;
                                }
                            };
                        } else if value == max_value {
                            self.delete_from_tree(value);
                            let last_populated = self.aux[0].maximum();
                            match last_populated {
                                Some(last_cluster) => {
                                    let new_max = self
                                        .children[last_cluster as usize]
                                        .maximum();
                                    match new_max {
                                        Some(max) => {
                                            self.max = Some((last_cluster
                                                * self.children.len() as T
                                                + max));
                                            return;
                                        },
                                        None => {
                                            panic!("Data structure appears corrupt");
                                        }
                                    };
                                },
                                None => {
                                    self.max = None;
                                    self.min = None;
                                }
                            };
                        } else if value < max_value && value > min_value {
                            self.delete_from_tree(value);
                            return;
                        } else {
                            return;
                        }

                    },
                    None => {
                        self.min = None;
                        self.max = None;
                        return;
                    }
                };
            },
            None => {
                self.min = None;
                self.max = None;
                return;
            }
        };
    }

    pub fn minimum(&self) -> Option<T> {
        return self.min;
    }

    pub fn maximum(&self) -> Option<T> {
        return self.max;
    }

    pub fn findnext(&self, value: T) -> Option<T> {
        if self.children.len() == 0 {
            let min_val = self.min?;
            let max_val = self.max?;
            if value == 0 && max_val == 1 {
                return self.max;
            } else {
                return None;
            }
        } else {
            match self.min {
                Some(min_value) => {
                    if value < min_value {
                        return self.min
                    }
                },
                None => ()
            };
            let cur_cluster_max = self.children[self.high(value)].maximum();
            match cur_cluster_max {
                Some(max_value) => {
                    if self.low(value) < max_value {
                        let offset = self.children[self.high(value)]
                            .findnext(self.low(value))?;
                        return match self.children[self.high(value)].search(offset) {
                            Some(n) => {
                                return Some(n + self.high(value)*self.children.len());
                            },
                            None => {
                                return None;
                            }
                        };
                    }
                },
                None => ()
            };
            let next_cluster = self.aux[0].findnext(self.high(value))?;
            let offset = self.children[next_cluster].minimum()?;
            return match self.children[next_cluster].search(offset) {
                Some(n) => {
                    return Some(n + next_cluster*self.children.len());
                },
                None => {
                    return None;
                }
            };
        }
    }

    pub fn findprev(&self, value: T) -> Option<T> {
        if self.children.len() == 0 {
            let max_value = self.maximum()?;
            let min_value = self.minimum()?;
            if max_value == value && max_value != min_value {
                return self.min;
            } else {
                return None;
            }
        } else {
            match self.maximum() {
                Some(max_value) => {
                    if value > max_value {
                        return self.max;
                    }
                },
                None => ()
            };
            let cur_cluster_min = self.children[self.high(value)].minimum();
            match cur_cluster_min {
                Some(min_value) => {
                    if self.low(value) > min_value {
                        let offset = self.children[self.high(value)]
                            .findprev(self.low(value))?;
                        return match self.children[self.high(value)].search(offset) {
                            Some(n) => {
                                return Some(n + self.high(value)*self.children.len());
                            },
                            None => {
                                return None;
                            }
                        };
                    }
                },
                None => ()
            };
            let next_cluster = self.aux[0].findprev(self.high(value))?;
            let offset = self.children[next_cluster].maximum()?;
            return match self.children[next_cluster].search(offset) {
                Some(n) => {
                    return Some(n + next_cluster*self.children.len());
                },
                None => {
                    return None;
                }
            };
        }
    }
}
