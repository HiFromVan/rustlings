// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterators() {
        let my_fav_fruits = ["banana", "custard apple", "avocado", "peach", "raspberry"];

        // 1. 使用 .iter() 创建一个迭代器
        // 必须用 mut，因为调用 .next() 会改变迭代器内部的状态（指针移动）
        let mut fav_fruits_iterator = my_fav_fruits.iter();

        assert_eq!(fav_fruits_iterator.next().unwrap(), Some(&"banana").unwrap());
        assert_eq!(fav_fruits_iterator.next(), Some(&"custard apple")); // 替换第一个 todo
        assert_eq!(fav_fruits_iterator.next(), Some(&"avocado"));
        assert_eq!(fav_fruits_iterator.next(), Some(&"peach"));         // 替换第二个 todo
        assert_eq!(fav_fruits_iterator.next(), Some(&"raspberry"));
        assert_eq!(fav_fruits_iterator.next(), None);                  // 迭代结束返回 None
    }
}
