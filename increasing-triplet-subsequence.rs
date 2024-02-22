fn main(){
	let nums = Vec::from([1,2,3,4,5]);
	let output = true;
	assert_eq!(output, increasing_triplet(nums));
}

fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut first = i32::MAX;
        let mut second = i32::MAX;

        // Very primitive solution, we just iteratively update the min, middle and max
        for element in nums{
            // If element is less than or equal to first, element becomes new first
            if element < first {
                first = element;  
            }
            // If element is greater than first but less than or equal to second,
            // we have found middle value
            if element > first && element < second{
                second = element;
            }
            // If element is smaller then first, but greater then second,
            // we have found triplet that satisfies condition
            if element > first && element > second{
                return true;
            }
        }

        return false;
    }