
fn median(mut numbers: Vec<f32>) -> Option<f32> { 
    let l = numbers.len();

    if l > 0 {
        numbers.sort_by(|x, y| x.partial_cmp(y).unwrap());
        let hl = (l as i32 / 2) as usize;

        Some(
            if l % 2 == 0 {
                (numbers[hl-1]+ numbers[hl]) / 2.0
            } else {
                numbers[hl]
            }
        )
    } else {
        None
    }
}

#[cfg(test)]
#[test]
fn test_median() {
    assert_eq!(Some(4.0), median(vec![1.5,3.0,5.0,8.8]));
    assert_eq!(Some(4.0), median(vec![1.5,4.0,5.0]));
    assert_eq!(None, median(vec!()));
}