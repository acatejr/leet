#[allow(dead_code)]

/**
 * 
 */
#[allow(dead_code)]
fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut results: Vec<i32> = Vec::new();
    results.push(nums[0]);

    for i in 0..nums.len()-1 {
        let sum: i32 = results[i] + nums[i+1];
        results.push(sum);
    }
    return results;
}

/**
 * 
 */
#[allow(dead_code)]
fn fizz_buzz(n: i32) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let p = 4;

    if 1 <= n && n <= i32::pow(p, 10) {
        for i in 1..n+1 {
            if i % 15 == 0 {
                result.push("FizzBuzz".to_string());
            } else if i % 3 == 0 {
                result.push("Fizz".to_string());
            } else if i % 5 == 0 {
                result.push("Buzz".to_string())
            } else {
                result.push(i.to_string());
            }
        }
    }

    return result;
}

/**
 * TODO: This can also be done with bitwise operations
 */
fn number_of_steps(num: i32) -> i32 {
    let mut steps = 0;
    let mut numerator = num;

    if 0 <= num && num <= 1000000 {
        while numerator > 0 {
            if numerator % 2 == 0 {
                numerator /=  2;
            } else {
                numerator -= 1;
            }
            steps += 1;
        }
    }
    return steps;
}

fn main() {

    println!("{}", number_of_steps(14));
    println!("{}", number_of_steps(8));
    println!("{}", number_of_steps(123));
    // println!("{:?}", fizz_buzz(3));
    // println!("{:?}", fizz_buzz(5));
    // println!("{:?}", fizz_buzz(15));

    // let mut nums = vec![1,2,3,4];
    // println!("{:?}", running_sum(nums));

    // nums = vec![1,1,1,1,1];
    // println!("{:?}", running_sum(nums));
 
    // nums = vec![3,1,2,10,1];
    // println!("{:?}", running_sum(nums));
}
