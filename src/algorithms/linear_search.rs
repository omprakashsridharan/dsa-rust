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

    pub fn linear_many_with_criteria(&self, predicate: fn(S) -> bool) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        for (index, value) in self.data.clone().enumerate() {
            if predicate(value) {
                result.push(index as i32)
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
        let search_index = search.linear(&2);
        assert_eq!(search_index, 1)
    }

    #[test]
    fn linear_not_exists() {
        let data = vec![1, 2, 3];
        let search = Search { data: data.iter() };
        let search_index = search.linear(&4);
        assert_eq!(search_index, -1)
    }

    #[test]
    fn linear_many_with_criteria() {
        let data = vec![1, 2, 3, 4];
        let search = Search { data: data.iter() };
        let search_index = search.linear_many_with_criteria(|v| v % 2 == 0);
        assert_eq!(search_index, vec![1, 3])
    }

    #[test]
    fn linear_many_with_criteria_not_exists() {
        let data = vec![1, 3, 5, 7];
        let search = Search { data: data.iter() };
        let search_index = search.linear_many_with_criteria(|v| v % 2 == 0);
        assert_eq!(search_index, vec![])
    }
}
