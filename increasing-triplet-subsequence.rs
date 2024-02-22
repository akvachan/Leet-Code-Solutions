fn main(){
	let nums = Vec::from([1,2,3,4,5]);
	let output = true;
	assert_eq!(output, increasing_triplet(nums));
}

fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut min = i32::MAX;
        let mut middle = i32::MAX;

        // Very primitive solution, we just iteratively update the min and middle
        for element in nums{
            // If element is less than min,
	    // we have found the new min value
            if element < min {
                min = element;  
            }
            // If element is greater than min but less than middle,
            // we have found the new middle value
            if element > min && element < middle{
                middle = element;
            }
            // If element is greater then min and greater than middle,
            // we have found the third and last needed value
            if element > min && element > middle{
                return true;
            }
        }

        return false;
    }
