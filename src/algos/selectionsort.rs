use crate::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        for i in 0..slice.len() {
            let least = i;

            for j in i + 1..slice.len() {
                if slice[j] < slice[least] {
                    slice.swap(j, least);
                }
            }
        }
    }
}
#[test]
fn selection_works() {
    let mut things = vec![11, 25, 12, 22, 64];

    crate::sort::<_, SelectionSort>(&mut things);

    assert_eq!(things, &[11, 12, 22, 25, 64]);
}
