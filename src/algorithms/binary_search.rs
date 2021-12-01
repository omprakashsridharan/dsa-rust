fn binary_search_helper<T>(data: &[T], search_item: T, left: usize, right: usize) -> Option<usize>
where
    T: Eq + Copy + PartialOrd,
{
    let mid = (right - left) / 2;
    if right >= left {
        if data[mid].eq(&search_item) {
            return Some(mid);
        }

        if data[mid] < data[mid] {
            return binary_search_helper(data, search_item, left, mid - 1);
        }
        if data[mid] > data[mid] {
            return binary_search_helper(data, search_item, mid + 1, right);
        }
    }
    return None;
}

pub fn binary_search<T>(data: &[T], search_item: T) -> Option<usize>
where
    T: Eq + Copy + PartialOrd,
{
    return binary_search_helper(data, search_item, 0, data.len() - 1);
}

#[cfg(test)]
mod test {
    use crate::algorithms::binary_search::binary_search;

    #[test]
    fn binary_search_exists() {
        let data = [1, 2, 3, 5, 6, 7];
        let search_index = binary_search(&data, 3);
        assert_eq!(search_index, Some(2))
    }

    #[test]
    fn binary_search_not_exists() {
        let data = [1, 2, 3, 5, 6, 7];
        let search_index = binary_search(&data, 10);
        assert_eq!(search_index, None)
    }
}
