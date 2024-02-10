fn main() {
    let candies = Vec::from([2,3,5,1,3]);
    let extra_candies = 3;
    let vals = kids_with_candies(candies, extra_candies);
    assert_eq!(vals, Vec::from([true,true,true,false,true]));
}

// Find maximum of array
// Then for each elements look if element + extra_candies is bigger than maximum
// if true, write true, else false
// No need to handle edge cases, since length of candies vector is bigger or equal than 2
// and extra_candies is always bigger or equal than 1
fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {

    let max = candies.iter().max().unwrap();
    let mut result = Vec::new();

    for x in &candies {
        if x + extra_candies >= *max {
            result.push(true);
        } else {
            result.push(false);
        }
    }
    
    return result;
}