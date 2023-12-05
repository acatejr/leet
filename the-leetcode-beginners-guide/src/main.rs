fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut results: Vec<i32> = Vec::new();
    results.push(nums[0]);

    for i in 0..nums.len()-1 {
        let sum: i32 = results[i] + nums[i+1];
        results.push(sum);
    }
    return results;
}

fn main() {
    let mut nums = vec![1,2,3,4];
    println!("{:?}", running_sum(nums));

    nums = vec![1,1,1,1,1];
    println!("{:?}", running_sum(nums));
 
    nums = vec![3,1,2,10,1];
    println!("{:?}", running_sum(nums));
}
