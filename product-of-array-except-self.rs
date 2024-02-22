fn main(){
	let nums: Vec<i32> = Vec::from([1,2,3,4]);
	let output: Vec<i32> = Vec::from([24,12,8,6]);
	assert_eq!(output, product_except_self(nums));
}

fn product_except_self(nums: Vec<i32>) -> Vec<i32> {

        let n = nums.len();
        let mut result: Vec<i32> = vec![1; n];
        let mut i = 0;

        while i <= n - 1 {
            if i != 0 {
                result[i] = nums[i-1]*result[i-1];
            }
            i += 1;
        }

        i = n - 1;
        let mut inter = 1;
        while i > 0 {
            inter *= nums[i];
            result[i-1] *= inter;
            i -= 1;
        }

        return result;

}