use std::cmp::Ordering;
use std::fmt;

// Ideally would use a union here, but ran into issues with union initialization and drop,
// so use a struct of optional values instead.
#[derive(Clone, Eq)]
pub struct Elem {
    pub list: Option<Vec<Elem>>,
    pub number: Option<u32>,
    pub divider: bool,
}

impl fmt::Debug for Elem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(l) = &self.list {
            return write!(f, "{:?}", l);
        } else if let Some(n) = self.number {
            return write!(f, "{:?}", n);
        } else {
            return fmt::Result::Err(fmt::Error);
        }
    }
}

fn cmp_elem_list(self_list: &Vec<Elem>, other_list: &Vec<Elem>) -> Ordering {
    for (self_val, other_val) in self_list.iter().zip(other_list.iter()) {
        let cmp = self_val.cmp(other_val);
        if cmp != Ordering::Equal {
            return cmp;
        }
    }

    return self_list.len().cmp(&other_list.len());
}

impl Ord for Elem {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.number.is_some() && other.number.is_some() {
            return self.number.unwrap().cmp(&other.number.unwrap());
        }

        if self.list.is_some() && other.list.is_some() {
            return cmp_elem_list(&self.list.as_ref().unwrap(), other.list.as_ref().unwrap());
        }

        if self.list.is_some() && other.number.is_some() {
            let tmp = vec![Elem {
                list: None,
                number: Some(other.number.unwrap()),
                divider: false,
            }];
            return cmp_elem_list(&self.list.as_ref().unwrap(), &tmp);
        }

        if self.number.is_some() && other.list.is_some() {
            let tmp = vec![Elem {
                list: None,
                number: Some(self.number.unwrap()),
                divider: false,
            }];
            return cmp_elem_list(&tmp, &other.list.as_ref().unwrap());
        }

        panic!("Invalid elem");
    }
}

impl PartialOrd for Elem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Elem {
    fn eq(&self, other: &Self) -> bool {
        self.list == other.list && self.number == other.number
    }
}
