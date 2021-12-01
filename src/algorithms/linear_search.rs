use std::cmp::PartialEq;
use std::fmt::Debug;

struct Search<S, T>
where
    T: Iterator<Item = S> + Clone,
    S: Debug + PartialEq,
{
    data: T,
}

impl<S, T> Search<S, T>
where
    T: Iterator<Item = S> + Clone,
    S: Debug + PartialEq,
{
    pub fn linear(&self, search_item: S) -> i32 {
        let mut result: i32 = -1;
        for (index, value) in self.data.clone().enumerate() {
            if value == search_item {
                result = index as i32
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Search;

    #[test]
    fn linear_exists() {
        let data = vec![1, 2, 3];
        let search = Search { data: data.iter() };
        let search_instance = search.linear(&2);
        assert_eq!(search_instance, 1)
    }

    #[test]
    fn linear_non_exists() {
        let data = vec![1, 2, 3];
        let search = Search { data: data.iter() };
        let search_instance = search.linear(&4);
        assert_eq!(search_instance, -1)
    }
}
