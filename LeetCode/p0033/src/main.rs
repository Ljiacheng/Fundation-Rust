/// 33. 搜索旋转排序数组
///
/// 整数数组 nums 按升序排列，数组中的值 互不相同 。
/// 
/// 在传递给函数之前，nums 在预先未知的某个下标 k（0 <= k < nums.length）上进行了 旋转，使数组变为 [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]]（下标 从 0 开始 计数）。例如， [0,1,2,4,5,6,7] 在下标 3 处经旋转后可能变为 [4,5,6,7,0,1,2] 。
/// 
/// 给你 旋转后 的数组 nums 和一个整数 target ，如果 nums 中存在这个目标值 target ，则返回它的下标，否则返回 -1 。
///
/// 示例 1：
/// 
/// 输入：nums = [4,5,6,7,0,1,2], target = 0
/// 输出：4
/// 示例 2：
/// 
/// 输入：nums = [4,5,6,7,0,1,2], target = 3
/// 输出：-1
/// 示例 3：
/// 
/// 输入：nums = [1], target = 0
/// 输出：-1
/// 
/// 
/// 提示：
/// 
/// 1 <= nums.length <= 5000
/// -10^4 <= nums[i] <= 10^4
/// nums 中的每个值都 独一无二
/// 题目数据保证 nums 在预先未知的某个下标上进行了旋转
/// -10^4 <= target <= 10^4
/// 
/// 
/// 进阶：你可以设计一个时间复杂度为 O(log n) 的解决方案吗？

pub fn search(nums: Vec<i32>, target: i32) -> bool {
    if nums.len() == 1 {
        if nums[0] == target { return true; }
        else { return false; }
    }
    let mut l: usize = 0;
    let mut r: usize = nums.len() - 1;
    while l < r {
        let m = (l + r) / 2;
        if m == l || m == r { break; }
        if nums[m] > target {
            if nums[l] > target && nums[l] < nums[m] {
                l = m;
                continue;
            }
            r = m;
        } else if nums[m] < target {
            if nums[r] < target && nums[r] > nums[m] {
                r = m;
                continue;
            }
            l = m;
        } else { return true; }
    }
    if nums[l] == target || nums[r] == target { true }
    else { false }
}

fn main() {
    let v = vec![1,1,1,1,1,1,1,1,1,13,1,1,1,1,1,1,1,1,1,1,1,1];
    let target = 13;
    println!("{}", search(v, target));
}
