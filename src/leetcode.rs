#[allow(dead_code)]
mod leetcode {

    // Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
    // An input string is valid if:
    // Open brackets must be closed by the same type of brackets.
    // Open brackets must be closed in the correct order.
    // Every close bracket has a corresponding open bracket of the same type.
    
    // Got this so many time wrong.. and it was so easy!! Just a LIFO
    fn is_valid(s: String) -> bool {
        let mut valid = true;
        let mut buffer : Vec<char> = vec!();
                                            //idx = 0 = (), 1 = [], 2 = {}

        if s.len() % 2 == 1 { // odd number of chars, no mathcing brackets
            return false
        }
        for c in s.chars() {
            match c {
                '(' => buffer.push(c),
                '[' => buffer.push(c),
                '{' => buffer.push(c),
                ')' => match buffer.pop() {
                        None => { valid = false; break; },
                        Some(x) => if x != '(' { valid = false; break; },
                    },
                ']' => match buffer.pop() {
                        None => { valid = false; break; },
                        Some(x) => if x != '[' { valid = false; break; },
                    },
                '}' => match buffer.pop() {
                        None => { valid = false; break; },
                        Some(x) => if x != '{' { valid = false; break; },
                    },
                _ => (),
            }
                
        }
        if valid && buffer.len() == 0 {
            valid
        } else {
            false
        }
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res : Vec<i32> = vec!();
        let l = nums.len();

        for i in 0..l-1 {
            let n = nums[i];
            let tofind = target - n;
            let mut subset = nums[(i+1)..l].iter();
            match subset.position(|&x| x == tofind) {
                Some(p) => {
                        res.push(i as i32);
                        res.push((p+i+1) as i32);
                        break;
                    },
                _ => (),
            }
        }
        res
    }
}

#[cfg(test)]
mod test_leetcode {
    use super::leetcode;


    #[test]
    fn test_two_sum() {
        assert_eq!(vec![1,2],leetcode::two_sum(vec![3,2,4],6));
    }
}