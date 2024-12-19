use itertools::equal;

fn shuffle(array: &[u32]) -> Vec<u32>{
    let M = array.len()/2;
    let (start, end) = (&array[..M], &array[M..]);
    start.iter().zip(end.iter()).map(|(&x,&y)| vec![x, y]).flatten().collect::<Vec<_>>()
}

pub fn shuffle_count(n: u32) -> u32 {
    let first_array = (1..2*n+1).collect::<Vec<_>>();
    let mut array = shuffle(&first_array);
    let mut count = 1;
    while !equal(&first_array,&array) {
        array = shuffle(&array);
        count += 1;
    }
    count
}