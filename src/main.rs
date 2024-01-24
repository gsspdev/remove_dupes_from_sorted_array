fn main() {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut counter: usize = 0;
        let mut numbers: Vec<i32> = nums.clone();
        while counter < nums.len() as usize {
            if numbers[counter] == numbers[counter + 1] {
            numbers.remove(counter);
            counter +=1;
            } else {
            counter +=1;
            }
        }
        let k: i32 = numbers.len() as i32;
        return k
    }
}
