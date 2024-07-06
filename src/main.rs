pub struct Solution;



impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        if ((time/(n-1)& 1)) != 0
         {
            n-time % (n-1)
        }
        else {
            1+time % (n-1)
        }    
    }
}


fn main() {
    let my_pass = Solution::pass_the_pillow(5, 6);

    println!("{my_pass}");
}
