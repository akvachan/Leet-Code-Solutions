fn main() {
	let test_flowerbed1: Vec<i32>  = Vec::from([1,0,0,1,0,0]);
	let n: i32 = 1;
	assert_eq!(can_place_flowers(test_flowerbed1, n), true);
}

fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        // init count
        let mut count: i32 = 0;

        // create a mutable copy of flowerbed
        let mut s_flowerbed: Vec<i32> = flowerbed;

        // iterate over flowerbed
        for i in 0..s_flowerbed.len(){

            // if flowerbed at i is 0, then look left and right
            if s_flowerbed[i] == 0 {

                // check if left and right has zero, if yes, plant a 
flower and increment count
                // handle edge cases like pot at index 0 (has only right 
neighbor)
                // and also handle pot at index len()-1 (has only left 
neighbor)
                // Assumption is made that inexistant neighbors have zero 
values
                // so at i = -1, there is no flower and i = len() there is 
no flower
                if (i == 0 || s_flowerbed[i-1] == 0) && 
                (i == s_flowerbed.len()-1 || s_flowerbed[i+1] == 0) {
                    s_flowerbed[i] = 1;
                    count += 1;
                }
            }
        }

        return n <= count;


    }
