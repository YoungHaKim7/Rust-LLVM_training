use std::mem;

/// Equivalent to llvm::adl_begin()
pub fn adl_begin<R>(range: R) -> R::IntoIter
where
    R: IntoIterator,
{
    range.into_iter()
}

/// Equivalent to llvm::adl_end()
///
/// Rust iterators don't expose an "end iterator".
/// We consume the iterator and return the last item.
pub fn adl_end<R>(range: R) -> Option<R::Item>
where
    R: IntoIterator,
{
    range.into_iter().last()
}

/// Equivalent to llvm::adl_rbegin()
pub fn adl_rbegin<R>(range: R) -> std::iter::Rev<R::IntoIter>
where
    R: IntoIterator,
    R::IntoIter: DoubleEndedIterator,
{
    range.into_iter().rev()
}

/// Equivalent to llvm::adl_rend()
///
/// Rust has no reverse-end iterator.
/// Returns the first element from the reversed range.
pub fn adl_rend<R>(range: R) -> Option<R::Item>
where
    R: IntoIterator,
    R::IntoIter: DoubleEndedIterator,
{
    range.into_iter().rev().last()
}

/// Equivalent to llvm::adl_swap()
pub fn adl_swap<T>(lhs: &mut T, rhs: &mut T) {
    mem::swap(lhs, rhs);
}

/// Equivalent to llvm::adl_size()
pub fn adl_size<R>(range: R) -> usize
where
    R: IntoIterator,
    R::IntoIter: ExactSizeIterator,
{
    range.into_iter().len()
}

fn main() {
    // Example 1: adl_begin() - Get iterator from range
    let numbers = vec![1, 2, 3, 4, 5];
    let mut iter = adl_begin(&numbers);
    println!("adl_begin: {:?}", iter.next());

    // Example 2: adl_end() - Get last element
    let last = adl_end(&numbers);
    println!("adl_end (last element): {:?}", last);

    // Example 3: adl_rbegin() - Reverse iterator
    let mut rev_iter = adl_rbegin(&numbers);
    println!("adl_rbegin (first from reversed): {:?}", rev_iter.next());
    println!("adl_rbegin (second from reversed): {:?}", rev_iter.next());

    // Example 4: adl_rend() - Last element from reversed range (first from original)
    let first = adl_rend(&numbers);
    println!("adl_rend (first element): {:?}", first);

    // Example 5: adl_swap() - Swap two values
    let mut a = 10;
    let mut b = 20;
    println!("Before swap: a={}, b={}", a, b);
    adl_swap(&mut a, &mut b);
    println!("After swap: a={}, b={}", a, b);

    // Example 6: adl_size() - Get size of range
    let size = adl_size(&numbers);
    println!("adl_size (length): {}", size);

    println!("\n--- All examples completed ---");
}
