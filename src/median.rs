mod linkedin {
    use std::fmt::Debug;

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

    // Remove duplicates from a vector
    fn unique(v : &Vec<i32>) -> Vec<i32> {
        let mut res : Vec<i32> = vec!();

        for e in v {
            if !res.contains(&e) {
                res.push(*e);
            }
        }
        res
    }

    // Generic version
    fn unique_gen<T: Ord>(mut v: Vec<T>) -> Vec<T> {
        v.sort();
        v.dedup();
        v
    }

    // generic display info
    fn info1<T: std::fmt::Display>(t: &T) {
        println!("{}", *t);
    }

    fn info2<T: std::fmt::Debug>(t: &T) {
        println!("{:?}", *t);
    }

    fn info3<T: ToString>(t: &T) {
        println!("{}", t.to_string());
    }
    // save memory !!!
    fn info<T: AsRef<str>>(t: &T) {
        println!("{}", t.as_ref());
    }

    fn sort_usernames(mut users: Vec<&str>) -> Vec<&str> {
        todo!()
    }

    #[cfg(test)]
    mod test_linkedin {
        use super::median;
        #[test]
        fn test_median() {
            assert_eq!(Some(4.0), median(vec![1.5,3.0,5.0,8.8]));
            assert_eq!(Some(4.0), median(vec![1.5,4.0,5.0]));
            assert_eq!(None, median(vec!()));
        }

        use super::unique;
        #[test]
        fn test_unique() {
            let v1 = vec![1,4,5];
            assert_eq!(v1, unique(&v1));

            assert_eq!(vec!(1,3), unique(&vec![1,1,3]));
        }
    }
}