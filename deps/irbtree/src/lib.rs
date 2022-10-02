// Copyright 2017-2018 By tickdream125@hotmail.com, semtexzv@gmail.com
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cmp::{Ord, Ordering};
use std::fmt::{self, Debug};
use std::iter::{FromIterator, IntoIterator};
use std::mem::ManuallyDrop;
use std::ops::{Index, IndexMut};
use std::{marker, mem, ptr};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Color {
    Red,
    Blk,
}

struct RBTreeNode<K: Ord, V> {
    color: Color,
    left: NodePtr<K, V>,
    right: NodePtr<K, V>,
    parent: NodePtr<K, V>,

    keys: (K, K),
    value: V,
    max: K,
}

unsafe impl<K: Send + Ord, V: Send> Send for RBTreeNode<K, V> {}

impl<K: Ord, V> RBTreeNode<K, V> {
    #[inline]
    fn into_pair(self) -> ((K, K), V) {
        (self.keys, self.value)
    }
}

impl<K, V> Debug for RBTreeNode<K, V>
where
    K: Ord + Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "k:{:>7?},{:<7?} max:{:>5?} v:{:<5?} c:{:>6?}",
            self.keys.0, self.keys.1, self.max, self.value, self.color
        )
    }
}

/*****************NodePtr***************************/
#[derive(Debug)]
struct NodePtr<K: Ord, V>(*mut RBTreeNode<K, V>);

unsafe impl<K: Send + Ord, V: Send> Send for NodePtr<K, V> {}

impl<K: Ord, V> Clone for NodePtr<K, V> {
    fn clone(&self) -> NodePtr<K, V> {
        NodePtr(self.0)
    }
}

impl<K: Ord, V> Copy for NodePtr<K, V> {}

impl<K: Ord, V> Ord for NodePtr<K, V> {
    fn cmp(&self, other: &NodePtr<K, V>) -> Ordering {
        unsafe { (*self.0).keys.cmp(&(*other.0).keys) }
    }
}

impl<K: Ord, V> PartialOrd for NodePtr<K, V> {
    fn partial_cmp(&self, other: &NodePtr<K, V>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord, V> PartialEq for NodePtr<K, V> {
    fn eq(&self, other: &NodePtr<K, V>) -> bool {
        self.0 == other.0
    }
}

impl<K: Ord, V> Eq for NodePtr<K, V> {}

impl<K: Ord + Clone, V> NodePtr<K, V> {
    fn new(k1: K, k2: K, v: V) -> NodePtr<K, V> {
        let node = RBTreeNode {
            color: Color::Blk,
            left: NodePtr::null(),
            right: NodePtr::null(),
            parent: NodePtr::null(),
            keys: (k1, k2.clone()),
            max: k2,
            value: v,
        };
        NodePtr(Box::into_raw(Box::new(node)))
    }

    #[inline]
    fn set_color(&mut self, color: Color) {
        if self.is_null() {
            return;
        }
        unsafe {
            (*self.0).color = color;
        }
    }

    #[inline]
    fn set_red_color(&mut self) {
        self.set_color(Color::Red);
    }

    #[inline]
    fn set_black_color(&mut self) {
        self.set_color(Color::Blk);
    }

    #[inline]
    fn get_color(&self) -> Color {
        if self.is_null() {
            return Color::Blk;
        }
        unsafe { (*self.0).color }
    }

    #[inline]
    fn is_red_color(&self) -> bool {
        if self.is_null() {
            return false;
        }
        unsafe { (*self.0).color == Color::Red }
    }

    #[inline]
    fn is_black_color(&self) -> bool {
        if self.is_null() {
            return true;
        }
        unsafe { (*self.0).color == Color::Blk }
    }

    #[inline]
    fn is_left_child(&self) -> bool {
        self.parent().left() == *self
    }

    #[inline]
    fn is_right_child(&self) -> bool {
        self.parent().right() == *self
    }

    #[inline]
    fn min_node(self) -> NodePtr<K, V> {
        let mut temp = self.clone();
        while !temp.left().is_null() {
            temp = temp.left();
        }
        return temp;
    }

    #[inline]
    fn max_node(self) -> NodePtr<K, V> {
        let mut temp = self.clone();
        while !temp.right().is_null() {
            temp = temp.right();
        }
        return temp;
    }

    #[inline]
    fn next(self) -> NodePtr<K, V> {
        if !self.right().is_null() {
            self.right().min_node()
        } else {
            let mut temp = self;
            loop {
                if temp.parent().is_null() {
                    return NodePtr::null();
                }
                if temp.is_left_child() {
                    return temp.parent();
                }
                temp = temp.parent();
            }
        }
    }

    #[inline]
    fn prev(self) -> NodePtr<K, V> {
        if !self.left().is_null() {
            self.left().max_node()
        } else {
            let mut temp = self;
            loop {
                if temp.parent().is_null() {
                    return NodePtr::null();
                }
                if temp.is_right_child() {
                    return temp.parent();
                }
                temp = temp.parent();
            }
        }
    }

    #[inline]
    unsafe fn min_overlapping(self, k: &K) -> NodePtr<K, V> {
        let mut result = NodePtr::null();
        let mut n = self;
        if !n.is_null() && &(*n.0).max >= k {
            loop {
                if n.contains(k) {
                    result = n;
                    n = n.left();
                    if n.is_null() || &(*n.0).max < k {
                        break;
                    }
                } else {
                    let left = n.left();
                    if !left.is_null() && &(*left.0).max >= k {
                        n = left
                    } else {
                        if &(*n.0).keys.0 > k {
                            break;
                        }
                        n = n.right();
                        if n.is_null() || &(*n.0).max < k {
                            break;
                        }
                    }
                }
            }
        }
        result
    }

    #[inline]
    unsafe fn next_overlapping(self, k: &K) -> NodePtr<K, V> {
        let mut x = self;
        let mut ret = NodePtr::null();

        if !self.right().is_null() {
            ret = self.right().min_overlapping(k);
        }

        while !x.parent().is_null() && ret.is_null() {
            if x.is_left_child() {
                ret = if x.parent().contains(k) {
                    x.parent()
                } else {
                    x.parent().right().min_overlapping(k)
                }
            }
            x = x.parent();
        }
        return ret;
    }

    #[inline]
    fn set_parent(&mut self, parent: NodePtr<K, V>) {
        if self.is_null() {
            return;
        }
        unsafe { (*self.0).parent = parent }
    }

    #[inline]
    fn set_left(&mut self, left: NodePtr<K, V>) {
        if self.is_null() {
            return;
        }
        unsafe { (*self.0).left = left }
    }

    #[inline]
    fn set_right(&mut self, right: NodePtr<K, V>) {
        if self.is_null() {
            return;
        }
        unsafe { (*self.0).right = right }
    }

    #[inline]
    fn fix_max_single(&mut self) {
        if self.is_null() {
            return;
        }

        unsafe {
            let mut max = &(*self.0).keys.1;
            if !self.left().is_null() {
                max = std::cmp::max(max, &(*self.left().0).max);
            }
            if !self.right().is_null() {
                max = std::cmp::max(max, &(*self.right().0).max);
            }
            if &(*self.0).max != max {
                (*self.0).max = max.clone();
            }
        }
    }

    #[inline]
    fn fix_max_propagate(&mut self) {
        if self.is_null() {
            return;
        }

        unsafe {
            let mut max = &(*self.0).keys.1;
            if !self.left().is_null() {
                max = std::cmp::max(max, &(*self.left().0).max);
            }
            if !self.right().is_null() {
                max = std::cmp::max(max, &(*self.right().0).max);
            }
            if &(*self.0).max != max {
                (*self.0).max = max.clone();
                self.parent().fix_max_propagate();
            }
        }
    }

    #[inline]
    fn parent(&self) -> NodePtr<K, V> {
        if self.is_null() {
            return NodePtr::null();
        }
        unsafe { (*self.0).parent.clone() }
    }

    #[inline]
    fn left(&self) -> NodePtr<K, V> {
        if self.is_null() {
            return NodePtr::null();
        }
        unsafe { (*self.0).left.clone() }
    }

    #[inline]
    fn right(&self) -> NodePtr<K, V> {
        if self.is_null() {
            return NodePtr::null();
        }
        unsafe { (*self.0).right.clone() }
    }

    #[inline]
    fn contains(&self, k: &K) -> bool {
        if self.is_null() {
            return false;
        }
        unsafe { &(*self.0).keys.0 <= k && k <= &(*self.0).keys.1 }
    }

    #[inline]
    fn null() -> NodePtr<K, V> {
        NodePtr(ptr::null_mut())
    }

    #[inline]
    fn is_null(&self) -> bool {
        self.0.is_null()
    }
}

impl<K: Ord + Clone, V: Clone> NodePtr<K, V> {
    unsafe fn deep_clone(&self) -> NodePtr<K, V> {
        let mut node = NodePtr::new(
            (*self.0).keys.0.clone(),
            (*self.0).keys.1.clone(),
            (*self.0).value.clone(),
        );
        if !self.left().is_null() {
            node.set_left(self.left().deep_clone());
            node.left().set_parent(node);
        }
        if !self.right().is_null() {
            node.set_right(self.right().deep_clone());
            node.right().set_parent(node);
        }
        node
    }
}

/// A red black tree implemented with Rust
/// It is required that the keys implement the [`Ord`] traits.
pub struct RBTree<K: Ord + Clone, V> {
    root: NodePtr<K, V>,
    len: usize,
}

// Drop all owned pointers if the tree is dropped
impl<K: Ord + Clone, V> Drop for RBTree<K, V> {
    #[inline]
    fn drop(&mut self) {
        self.clear();
    }
}

impl<K: Ord + Clone, V> Default for RBTree<K, V> {
    fn default() -> Self {
        RBTree::new()
    }
}

/// If key and value are both impl Clone, we can call clone to get a copy.
impl<K: Ord + Clone, V: Clone> Clone for RBTree<K, V> {
    fn clone(&self) -> RBTree<K, V> {
        unsafe {
            let mut new = RBTree::new();
            new.root = self.root.deep_clone();
            new.len = self.len;
            new
        }
    }
}

impl<K, V> Debug for RBTree<K, V>
where
    K: Ord + Clone + Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

/// This is a method to help us to get inner struct.
impl<K: Ord + Debug + Clone, V: Debug> RBTree<K, V> {
    fn tree_print(&self, node: NodePtr<K, V>, direction: i32) {
        if node.is_null() {
            return;
        }
        if direction == 0 {
            unsafe {
                println!("{:?} is root node", (*node.0));
            }
        } else {
            let direct = if direction == -1 { "left" } else { "right" };
            unsafe {
                println!(
                    "{:?} is {:?}'s {:?} child ",
                    (*node.0),
                    *node.parent().0,
                    direct
                );
            }
        }
        self.tree_print(node.left(), -1);
        self.tree_print(node.right(), 1);
    }

    pub fn print_tree(&self) {
        if self.root.is_null() {
            println!("This is a empty tree");
            return;
        }
        println!("This tree size = {:?}, begin:-------------", self.len());
        self.tree_print(self.root, 0);
        println!("end--------------------------");
    }
}

/// all key be same, but it has multi key, if has multi key, it perhaps no correct
impl<K, V> PartialEq for RBTree<K, V>
where
    K: Eq + Ord + Clone,
    V: PartialEq,
{
    fn eq(&self, other: &RBTree<K, V>) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter()
            .all(|((k1, k2), value)| other.get(k1, k2).map_or(false, |v| *value == *v))
    }
}

impl<K, V> Eq for RBTree<K, V>
where
    K: PartialEq + Eq + Ord + Clone,
    V: PartialEq + Eq,
{
}

impl<'a, K, V> Index<(&'a K, &'a K)> for RBTree<K, V>
where
    K: Ord + Clone,
{
    type Output = V;

    #[inline]
    fn index(&self, index: (&K, &K)) -> &V {
        self.get(index.0, index.1).expect("no entry found for key")
    }
}

impl<'a, K, V> IndexMut<(&'a K, &'a K)> for RBTree<K, V>
where
    K: Ord + Clone,
{
    #[inline]
    fn index_mut(&mut self, index: (&K, &K)) -> &mut V {
        self.get_mut(index.0, index.1)
            .expect("no entry found for key")
    }
}

impl<'a, K, V> Index<&'a (K, K)> for RBTree<K, V>
where
    K: Ord + Clone,
{
    type Output = V;

    #[inline]
    fn index(&self, index: &(K, K)) -> &V {
        self.get(&index.0, &index.1)
            .expect("no entry found for key")
    }
}

impl<'a, K, V> IndexMut<&'a (K, K)> for RBTree<K, V>
where
    K: Ord + Clone,
{
    #[inline]
    fn index_mut(&mut self, index: &(K, K)) -> &mut V {
        self.get_mut(&index.0, &index.1)
            .expect("no entry found for key")
    }
}

impl<K: Ord + Clone, V> FromIterator<(K, K, V)> for RBTree<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, K, V)>>(iter: T) -> RBTree<K, V> {
        let mut tree = RBTree::new();
        tree.extend(iter);
        tree
    }
}

impl<K: Ord + Clone, V> FromIterator<((K, K), V)> for RBTree<K, V> {
    fn from_iter<T: IntoIterator<Item = ((K, K), V)>>(iter: T) -> RBTree<K, V> {
        let mut tree = RBTree::new();
        tree.extend(iter.into_iter().map(|(k, v)| (k.0, k.1, v)));
        tree
    }
}

/// RBTree into iter
impl<K: Ord + Clone, V> Extend<(K, K, V)> for RBTree<K, V> {
    fn extend<T: IntoIterator<Item = (K, K, V)>>(&mut self, iter: T) {
        let iter = iter.into_iter();
        for (k1, k2, v) in iter {
            self.insert(k1, k2, v);
        }
    }
}

/// provide the rbtree all keys
/// # Examples
/// ```
/// use irbtree::RBTree;
/// let mut m = RBTree::new();
/// for i in 1..6 {
///     m.insert(i, i, i);
/// }
/// let vec = vec![(1, 1), (2, 2), (3, 3), (4, 4), (5, 5)];
/// let key_vec: Vec<_> = m.keys().cloned().collect();
/// assert_eq!(vec, key_vec);
/// ```
pub struct Keys<'a, K: Ord + 'a, V: 'a> {
    inner: Iter<'a, K, V>,
}

impl<'a, K: Ord, V> Clone for Keys<'a, K, V> {
    fn clone(&self) -> Keys<'a, K, V> {
        Keys {
            inner: self.inner.clone(),
        }
    }
}

impl<'a, K: Ord + Clone + Debug, V> fmt::Debug for Keys<'a, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<'a, K: Ord + Clone, V> Iterator for Keys<'a, K, V> {
    type Item = &'a (K, K);

    #[inline]
    fn next(&mut self) -> Option<&'a (K, K)> {
        self.inner.next().map(|(k, _)| k)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

/// provide the rbtree all values order by key
/// # Examples
/// ```
/// use irbtree::RBTree;
/// let mut m = RBTree::new();
/// m.insert(2, 3, 5);
/// m.insert(1, 2, 6);
/// m.insert(3, 4, 8);
/// m.insert(4, 5, 9);
/// let vec = vec![6, 5, 8, 9];
/// let key_vec: Vec<_> = m.values().cloned().collect();
/// assert_eq!(vec, key_vec);
/// ```
pub struct Values<'a, K: 'a + Ord, V: 'a> {
    inner: Iter<'a, K, V>,
}

impl<'a, K: Ord, V> Clone for Values<'a, K, V> {
    fn clone(&self) -> Values<'a, K, V> {
        Values {
            inner: self.inner.clone(),
        }
    }
}

impl<'a, K: Ord + Clone + Debug, V: Debug> fmt::Debug for Values<'a, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<'a, K: Ord + Clone, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    #[inline]
    fn next(&mut self) -> Option<&'a V> {
        self.inner.next().map(|(_, v)| v)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

/// provide the rbtree all values and it can be modify
/// # Examples
/// ```
/// use irbtree::RBTree;
/// let mut m = RBTree::new();
/// for i in 0..32 {
///     m.insert(i, i, i);
/// }
/// assert_eq!(m.len(), 32);
/// for v in m.values_mut() {
///     *v *= 2;
/// }
/// for i in 0..32 {
///     assert_eq!(m.get(&i, &i).unwrap(), &(i * 2));
/// }
/// ```
pub struct ValuesMut<'a, K: 'a + Ord, V: 'a> {
    inner: IterMut<'a, K, V>,
}

impl<'a, K: Ord, V> Clone for ValuesMut<'a, K, V> {
    fn clone(&self) -> ValuesMut<'a, K, V> {
        ValuesMut {
            inner: self.inner.clone(),
        }
    }
}

impl<'a, K: Ord + Clone + Debug, V: Debug> fmt::Debug for ValuesMut<'a, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<'a, K: Ord + Clone, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    #[inline]
    fn next(&mut self) -> Option<&'a mut V> {
        self.inner.next().map(|(_, v)| v)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

/// Convert RBTree to iter, move out the tree.
pub struct IntoIter<K: Ord + Clone, V> {
    head: NodePtr<K, V>,
    tail: NodePtr<K, V>,
    len: usize,
    _d: ManuallyDrop<RBTree<K, V>>,
}

// Drop all owned pointers if the collection is dropped
impl<K: Ord + Clone, V> Drop for IntoIter<K, V> {
    #[inline]
    fn drop(&mut self) {
        while let Some(..) = self.next() {}
        let root = self._d.root;
        self._d.clear_recurse_nodrop(root);
        // unsafe { ManuallyDrop::drop(&mut self._d) }
    }
}

impl<K: Ord + Clone, V> Iterator for IntoIter<K, V> {
    type Item = ((K, K), V);

    fn next(&mut self) -> Option<((K, K), V)> {
        if self.len == 0 {
            return None;
        }

        if self.head.is_null() {
            return None;
        }
        let next = self.head.next();
        let (k, v, _) = unsafe {
            (
                core::ptr::read(&(*self.head.0).keys),
                core::ptr::read(&(*self.head.0).value),
                // Read the max, dropping it in the process, since we won't be actually dropping the nodes
                core::ptr::read(&(*self.head.0).max),
            )
        };
        self.head = next;
        self.len -= 1;
        Some((k, v))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<K: Ord + Clone, V> DoubleEndedIterator for IntoIter<K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<((K, K), V)> {
        if self.len == 0 {
            return None;
        }

        if self.tail.is_null() {
            return None;
        }

        let prev = self.tail.prev();
        let obj = unsafe { Box::from_raw(self.tail.0) };
        let (k, v) = obj.into_pair();
        self.tail = prev;
        self.len -= 1;
        Some((k, v))
    }
}

/// provide iter ref for RBTree
/// # Examples
/// ```
/// use irbtree::RBTree;
/// let mut m = RBTree::new();
/// for i in 0..32 {
///     m.insert(i, i,  i * 2);
/// }
/// assert_eq!(m.len(), 32);
/// let mut observed: u32 = 0;
/// for ((s, e), v) in m.iter() {
///     assert_eq!(*v, s * 2);
///     observed |= 1 << s;
/// }
/// assert_eq!(observed, 0xFFFF_FFFF);
/// ```
pub struct Iter<'a, K: Ord + 'a, V: 'a> {
    head: NodePtr<K, V>,
    tail: NodePtr<K, V>,
    len: usize,
    _marker: marker::PhantomData<&'a ()>,
}

impl<'a, K: Ord + 'a, V: 'a> Clone for Iter<'a, K, V> {
    fn clone(&self) -> Iter<'a, K, V> {
        Iter {
            head: self.head,
            tail: self.tail,
            len: self.len,
            _marker: self._marker,
        }
    }
}

impl<'a, K: Ord + Clone + 'a, V: 'a> Iterator for Iter<'a, K, V> {
    type Item = (&'a (K, K), &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }

        if self.head.is_null() {
            return None;
        }

        let (k, v) = unsafe { (&(*self.head.0).keys, &(*self.head.0).value) };
        self.head = self.head.next();
        self.len -= 1;
        Some((k, v))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, K: Ord + Clone + 'a, V: 'a> DoubleEndedIterator for Iter<'a, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<(&'a (K, K), &'a V)> {
        // println!("len = {:?}", self.len);
        if self.len == 0 {
            return None;
        }

        if self.tail == self.head {
            return None;
        }

        let (k, v) = unsafe { (&(*self.tail.0).keys, &(*self.tail.0).value) };
        self.tail = self.tail.prev();
        self.len -= 1;
        Some((k, v))
    }
}

/// provide iter mut ref for RBTree
/// # Examples
/// ```
/// use irbtree::RBTree;
/// let mut m = RBTree::new();
/// for i in 0..32 {
///     m.insert(i, i, i);
/// }
/// assert_eq!(m.len(), 32);
/// for (_, v) in m.iter_mut() {
///     *v *= 2;
/// }
/// for i in 0..32 {
///     assert_eq!(m.get(&i, &i).unwrap(), &(i * 2));
/// }
/// ```
pub struct IterMut<'a, K: Ord + 'a, V: 'a> {
    head: NodePtr<K, V>,
    tail: NodePtr<K, V>,
    len: usize,
    _marker: marker::PhantomData<&'a ()>,
}

impl<'a, K: Ord + 'a, V: 'a> Clone for IterMut<'a, K, V> {
    fn clone(&self) -> IterMut<'a, K, V> {
        IterMut {
            head: self.head,
            tail: self.tail,
            len: self.len,
            _marker: self._marker,
        }
    }
}

impl<'a, K: Ord + Clone + 'a, V: 'a> Iterator for IterMut<'a, K, V> {
    type Item = (&'a (K, K), &'a mut V);

    fn next(&mut self) -> Option<(&'a (K, K), &'a mut V)> {
        if self.len == 0 {
            return None;
        }

        if self.head.is_null() {
            return None;
        }

        let (k, v) = unsafe { ((&(*self.head.0).keys), &mut (*self.head.0).value) };
        self.head = self.head.next();
        self.len -= 1;
        Some((k, v))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, K: Ord + Clone + 'a, V: 'a> DoubleEndedIterator for IterMut<'a, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<(&'a (K, K), &'a mut V)> {
        if self.len == 0 {
            return None;
        }

        if self.tail == self.head {
            return None;
        }

        let (k, v) = unsafe { ((&(*self.tail.0).keys), &mut (*self.tail.0).value) };
        self.tail = self.tail.prev();
        self.len -= 1;
        Some((k, v))
    }
}

// We're bit-copying in the IntoIter impl, Unpin guarantees we don't screw something up
impl<K: Ord + Clone + Unpin, V: Unpin> IntoIterator for RBTree<K, V> {
    type Item = ((K, K), V);
    type IntoIter = IntoIter<K, V>;

    #[inline]
    fn into_iter(self) -> IntoIter<K, V> {
        let iter = if self.root.is_null() {
            IntoIter {
                head: NodePtr::null(),
                tail: NodePtr::null(),
                len: self.len,
                _d: ManuallyDrop::new(self),
            }
        } else {
            IntoIter {
                head: self.first_child(),
                tail: self.last_child(),
                len: self.len,
                _d: ManuallyDrop::new(self),
            }
        };
        iter
    }
}

pub struct Overlaps<'a, K: Ord + 'a, V: 'a> {
    k: &'a K,
    node: NodePtr<K, V>,
    _marker: marker::PhantomData<&'a ()>,
}

impl<'a, K: Ord + Clone, V> Iterator for Overlaps<'a, K, V> {
    type Item = (&'a (K, K), &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.node.is_null() {
            return None;
        }
        unsafe {
            let ret = (&(*self.node.0).keys, &(*self.node.0).value);
            self.node = self.node.next_overlapping(self.k);
            Some(ret)
        }
    }
}

pub struct OverlapsMut<'a, K: Ord + 'a, V: 'a> {
    k: &'a K,
    node: NodePtr<K, V>,
    _marker: marker::PhantomData<&'a ()>,
}

impl<'a, K: Ord + Clone, V> Iterator for OverlapsMut<'a, K, V> {
    type Item = (&'a (K, K), &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.node.is_null() {
            return None;
        }
        unsafe {
            let ret = (&(*self.node.0).keys, &mut (*self.node.0).value);
            self.node = self.node.next_overlapping(self.k);
            Some(ret)
        }
    }
}

impl<K: Ord + Clone, V> RBTree<K, V> {
    /// Creates an empty `RBTree`.
    pub fn new() -> RBTree<K, V> {
        RBTree {
            root: NodePtr::null(),
            len: 0,
        }
    }

    /// Returns the len of `RBTree`.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the `RBTree` is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.root.is_null()
    }

    /*
     *      px                              px
     *     /                               /
     *    x                               y
     *   /  \      --( RL )-->           / \
     *  lx   y                          x  ry
     *     /   \                       /  \
     *    ly   ry                     lx  ly
     *
     *
     */
    #[inline]
    unsafe fn left_rotate(&mut self, mut x: NodePtr<K, V>) {
        let mut y = x.right();
        x.set_right(y.left());

        if !y.left().is_null() {
            y.left().set_parent(x.clone());
        }
        y.set_parent(x.parent());

        if x == self.root {
            self.root = y.clone();
        } else if x.is_left_child() {
            x.parent().set_left(y.clone());
        } else {
            x.parent().set_right(y.clone());
        }

        y.set_left(x.clone());
        x.set_parent(y.clone());

        x.fix_max_single();
        y.fix_max_single();
    }

    /*
     *            py                               py
     *           /                                /
     *          y                                x
     *         /  \      --( RR )-->            /  \                     #
     *        x   ry                           lx   y
     *       / \                                   / \                   #
     *      lx  rx                                rx  ry
     *
     */
    #[inline]
    unsafe fn right_rotate(&mut self, mut x: NodePtr<K, V>) {
        let mut y = x.left();
        x.set_left(y.right());

        if !y.right().is_null() {
            y.right().set_parent(x.clone());
        }

        y.set_parent(x.parent());
        if x == self.root {
            self.root = y.clone();
        } else if x == x.parent().right() {
            x.parent().set_right(y.clone());
        } else {
            x.parent().set_left(y.clone());
        }

        y.set_right(x.clone());
        x.set_parent(y.clone());

        x.fix_max_single();
        y.fix_max_single();
    }

    /// replace value if key exist, if not exist insert it.
    /// # Examples
    /// ```
    /// use irbtree::RBTree;
    /// let mut m = RBTree::new();
    /// assert_eq!(m.len(), 0);
    /// m.insert(2, 6, 4);
    /// assert_eq!(m.len(), 1);
    /// assert_eq!(m.insert(2, 6, 6).unwrap(), 4);
    /// assert_eq!(m.len(), 1);
    /// assert_eq!(*m.get(&2, &6).unwrap(), 6);
    /// ```
    #[inline]
    pub fn insert(&mut self, k1: K, k2: K, mut v: V) -> Option<V> {
        let node = self.find_node(&k1, &k2);
        if node.is_null() {
            self.do_insert(k1, k2, v);
            return None;
        }

        unsafe {
            mem::swap(&mut v, &mut (*node.0).value);
        }

        Some(v)
    }

    // #[inline]
    // unsafe fn insert_fixup(&mut self, mut z: NodePtr<K, V>) {
    //     let mut parent;
    //     let mut grandparent;
    //
    //     while z.parent().is_red_color() {
    //         parent = z.parent();
    //         grandparent = z.parent();
    //
    //         if z.parent().is_left_child() {
    //             let mut y = z.parent().parent().right();
    //             if y.is_red_color() {
    //                 z.parent().set_black_color();
    //                 y.set_black_color();
    //                 z.parent().parent().set_red_color();
    //                 z = z.parent().parent();
    //             } else {
    //                 if z.is_right_child() {
    //                     z = z.parent();
    //                     self.left_rotate(z)
    //                 }
    //                 z.parent().set_black_color();
    //                 z.parent().parent().set_red_color();
    //                 self.right_rotate(z.parent().parent());
    //             }
    //         } else {
    //             let mut y = z.parent().parent().left();
    //             if y.is_red_color() {
    //                 z.parent().set_black_color();
    //                 y.set_black_color();
    //                 z.parent().parent().set_red_color();
    //                 z = z.parent().parent();
    //             } else {
    //                 if z.is_left_child() {
    //                     z = z.parent();
    //                     self.right_rotate(z)
    //                 }
    //                 z.parent().set_black_color();
    //                 z.parent().parent().set_red_color();
    //                 self.left_rotate(z.parent().parent());
    //             }
    //         }
    //     }
    //     self.root.set_black_color();
    // }

    #[inline]
    unsafe fn insert_fixup(&mut self, mut node: NodePtr<K, V>) {
        let mut parent;
        let mut gparent;

        while node.parent().is_red_color() {
            parent = node.parent();
            gparent = parent.parent();
            //若“父节点”是“祖父节点的左孩子”
            if parent == gparent.left() {
                // Case 1条件：叔叔节点是红色
                let mut uncle = gparent.right();
                if !uncle.is_null() && uncle.is_red_color() {
                    uncle.set_black_color();
                    parent.set_black_color();
                    gparent.set_red_color();
                    node = gparent;
                    continue;
                }

                // Case 2条件：叔叔是黑色，且当前节点是右孩子
                if parent.right() == node {
                    self.left_rotate(parent);
                    let temp = parent;
                    parent = node;
                    node = temp;
                }

                // Case 3条件：叔叔是黑色，且当前节点是左孩子。
                parent.set_black_color();
                gparent.set_red_color();
                self.right_rotate(gparent);
            } else {
                // Case 1条件：叔叔节点是红色
                let mut uncle = gparent.left();
                if !uncle.is_null() && uncle.is_red_color() {
                    uncle.set_black_color();
                    parent.set_black_color();
                    gparent.set_red_color();
                    node = gparent;
                    continue;
                }

                // Case 2条件：叔叔是黑色，且当前节点是右孩子
                if parent.left() == node {
                    self.right_rotate(parent);
                    let temp = parent;
                    parent = node;
                    node = temp;
                }

                // Case 3条件：叔叔是黑色，且当前节点是左孩子。
                parent.set_black_color();
                gparent.set_red_color();
                self.left_rotate(gparent);
            }
        }
        self.root.set_black_color();
    }

    #[inline]
    fn do_insert(&mut self, k1: K, k2: K, v: V) {
        self.len += 1;

        let mut z = NodePtr::new(k1, k2, v);
        let mut y = NodePtr::null();
        let mut x = self.root;

        while !x.is_null() {
            y = x;
            unsafe {
                (*x.0).max = std::cmp::max(&(*x.0).max, &(*z.0).max).clone();
            }
            if z < x {
                x = x.left()
            } else {
                x = x.right()
            }
        }
        z.set_parent(y);

        if y.is_null() {
            self.root = z;
        } else if z < y {
            y.set_left(z);
        } else {
            y.set_right(z);
        }
        z.set_red_color();
        unsafe {
            self.insert_fixup(z);
        }
    }

    #[inline]
    fn find_node(&self, k1: &K, k2: &K) -> NodePtr<K, V> {
        if self.root.is_null() {
            return NodePtr::null();
        }
        let mut temp = &self.root;
        unsafe {
            loop {
                let tk = (&(*temp.0).keys.0, &(*temp.0).keys.1);
                let next = match (k1, k2).cmp(&tk) {
                    Ordering::Less => &mut (*temp.0).left,
                    Ordering::Greater => &mut (*temp.0).right,
                    Ordering::Equal => return *temp,
                };
                if next.is_null() {
                    break;
                }
                temp = next;
            }
        }
        NodePtr::null()
    }

    #[inline]
    fn first_child(&self) -> NodePtr<K, V> {
        if self.root.is_null() {
            NodePtr::null()
        } else {
            let mut temp = self.root;
            while !temp.left().is_null() {
                temp = temp.left();
            }
            return temp;
        }
    }

    #[inline]
    fn last_child(&self) -> NodePtr<K, V> {
        if self.root.is_null() {
            NodePtr::null()
        } else {
            let mut temp = self.root;
            while !temp.right().is_null() {
                temp = temp.right();
            }
            return temp;
        }
    }

    #[inline]
    pub fn get_first(&self) -> Option<(&(K, K), &V)> {
        let first = self.first_child();
        if first.is_null() {
            return None;
        }
        unsafe { Some((&(*first.0).keys, &(*first.0).value)) }
    }

    #[inline]
    pub fn get_last(&self) -> Option<(&(K, K), &V)> {
        let last = self.last_child();
        if last.is_null() {
            return None;
        }
        unsafe { Some((&(*last.0).keys, &(*last.0).value)) }
    }

    #[inline]
    pub fn pop_first(&mut self) -> Option<((K, K), V)> {
        let first = self.first_child();
        if first.is_null() {
            return None;
        }
        unsafe { Some(self.delete(first)) }
    }

    #[inline]
    pub fn pop_last(&mut self) -> Option<((K, K), V)> {
        let last = self.last_child();
        if last.is_null() {
            return None;
        }
        unsafe { Some(self.delete(last)) }
    }

    #[inline]
    pub fn get_first_mut(&mut self) -> Option<(&(K, K), &mut V)> {
        let first = self.first_child();
        if first.is_null() {
            return None;
        }
        unsafe { Some((&(*first.0).keys, &mut (*first.0).value)) }
    }

    #[inline]
    pub fn get_last_mut(&mut self) -> Option<(&(K, K), &mut V)> {
        let last = self.last_child();
        if last.is_null() {
            return None;
        }
        unsafe { Some((&(*last.0).keys, &mut (*last.0).value)) }
    }

    #[inline]
    pub fn get(&self, k1: &K, k2: &K) -> Option<&V> {
        let node = self.find_node(k1, k2);
        if node.is_null() {
            return None;
        }

        unsafe { Some(&(*node.0).value) }
    }

    #[inline]
    pub fn get_mut(&mut self, k1: &K, k2: &K) -> Option<&mut V> {
        let node = self.find_node(k1, k2);
        if node.is_null() {
            return None;
        }

        unsafe { Some(&mut (*node.0).value) }
    }

    #[inline]
    pub fn contains_key(&self, k: &K, k2: &K) -> bool {
        let node = self.find_node(k, k2);
        if node.is_null() {
            return false;
        }
        true
    }

    #[inline]
    fn clear_recurse(&mut self, current: NodePtr<K, V>) {
        if !current.is_null() {
            unsafe {
                self.clear_recurse(current.left());
                self.clear_recurse(current.right());
                Box::from_raw(current.0);
            }
        }
    }
    #[inline]
    fn clear_recurse_nodrop(&mut self, current: NodePtr<K, V>) {
        if !current.is_null() {
            unsafe {
                self.clear_recurse(current.left());
                self.clear_recurse(current.right());
                // Using manuallyDrop to disable dropping of the value. We're bit-copying them in IntoIter, so
                // we need to
                Box::from_raw(
                    std::mem::transmute::<_, *mut ManuallyDrop<RBTreeNode<K, V>>>(current.0),
                );
            }
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        let root = self.root;
        self.root = NodePtr::null();
        self.clear_recurse(root);
    }

    #[inline]
    pub fn clear_nodrop(&mut self) {
        let root = self.root;
        self.root = NodePtr::null();
        self.clear_recurse_nodrop(root);
    }

    #[inline]
    pub fn remove(&mut self, k: &K, k2: &K) -> Option<V> {
        let node = self.find_node(k, k2);
        if node.is_null() {
            return None;
        }
        unsafe { Some(self.delete(node).1) }
    }

    #[inline]
    unsafe fn delete_fixup(&mut self, mut node: NodePtr<K, V>, mut parent: NodePtr<K, V>) {
        let mut other;
        while node != self.root && node.is_black_color() {
            if parent.left() == node {
                other = parent.right();
                //x的兄弟w是红色的
                if other.is_red_color() {
                    other.set_black_color();
                    parent.set_red_color();
                    self.left_rotate(parent);
                    other = parent.right();
                }

                //x的兄弟w是黑色，且w的俩个孩子也都是黑色的
                if other.left().is_black_color() && other.right().is_black_color() {
                    other.set_red_color();
                    node = parent;
                    parent = node.parent();
                } else {
                    //x的兄弟w是黑色的，并且w的左孩子是红色，右孩子为黑色。
                    if other.right().is_black_color() {
                        other.left().set_black_color();
                        other.set_red_color();
                        self.right_rotate(other);
                        other = parent.right();
                    }
                    //x的兄弟w是黑色的；并且w的右孩子是红色的，左孩子任意颜色。
                    other.set_color(parent.get_color());
                    parent.set_black_color();
                    other.right().set_black_color();
                    self.left_rotate(parent);
                    node = self.root;
                    break;
                }
            } else {
                other = parent.left();
                //x的兄弟w是红色的
                if other.is_red_color() {
                    other.set_black_color();
                    parent.set_red_color();
                    self.right_rotate(parent);
                    other = parent.left();
                }

                //x的兄弟w是黑色，且w的俩个孩子也都是黑色的
                if other.left().is_black_color() && other.right().is_black_color() {
                    other.set_red_color();
                    node = parent;
                    parent = node.parent();
                } else {
                    //x的兄弟w是黑色的，并且w的左孩子是红色，右孩子为黑色。
                    if other.left().is_black_color() {
                        other.right().set_black_color();
                        other.set_red_color();
                        self.left_rotate(other);
                        other = parent.left();
                    }
                    //x的兄弟w是黑色的；并且w的右孩子是红色的，左孩子任意颜色。
                    other.set_color(parent.get_color());
                    parent.set_black_color();
                    other.left().set_black_color();
                    self.right_rotate(parent);
                    node = self.root;
                    break;
                }
            }
        }

        node.set_black_color();
    }

    #[inline]
    unsafe fn delete(&mut self, mut node: NodePtr<K, V>) -> ((K, K), V) {
        // TODO: no idea if the max fixups here are correct
        let mut child;
        let mut parent;
        let color;

        self.len -= 1;

        if !node.left().is_null() && !node.right().is_null() {
            let mut replace = node.right().min_node();
            if node == self.root {
                self.root = replace;
            } else {
                if node.parent().left() == node {
                    node.parent().set_left(replace);
                } else {
                    node.parent().set_right(replace);
                }
            }

            child = replace.right();
            parent = replace.parent();
            color = replace.get_color();
            if parent == node {
                parent = replace;
            } else {
                if !child.is_null() {
                    child.set_parent(parent);
                }
                parent.set_left(child);
                replace.set_right(node.right());
                node.right().set_parent(replace);
            }

            replace.set_parent(node.parent());
            replace.set_color(node.get_color());
            replace.set_left(node.left());
            node.left().set_parent(replace);

            if color == Color::Blk {
                self.delete_fixup(child, parent);
            }
            child.fix_max_propagate();

            let obj = Box::from_raw(node.0);
            return obj.into_pair();
        }

        if !node.left().is_null() {
            child = node.left();
        } else {
            child = node.right();
        }

        parent = node.parent();
        color = node.get_color();
        if !child.is_null() {
            child.set_parent(parent);
        }

        if self.root == node {
            self.root = child
        } else {
            if parent.left() == node {
                parent.set_left(child);
                node.fix_max_propagate();
            } else {
                parent.set_right(child);
                node.fix_max_propagate();
            }
        }

        if color == Color::Blk {
            self.delete_fixup(child, parent);
        }

        let obj = Box::from_raw(node.0);
        return obj.into_pair();
    }

    /// Return the keys iter
    #[inline]
    pub fn keys(&self) -> Keys<K, V> {
        Keys { inner: self.iter() }
    }

    /// Return the value iter
    #[inline]
    pub fn values(&self) -> Values<K, V> {
        Values { inner: self.iter() }
    }

    /// Return the value iter mut
    #[inline]
    pub fn values_mut(&mut self) -> ValuesMut<K, V> {
        ValuesMut {
            inner: self.iter_mut(),
        }
    }

    /// Return the key and value iter
    #[inline]
    pub fn iter(&self) -> Iter<K, V> {
        Iter {
            head: self.first_child(),
            tail: self.last_child(),
            len: self.len,
            _marker: marker::PhantomData,
        }
    }

    /// Return the key and mut value iter
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        IterMut {
            head: self.first_child(),
            tail: self.last_child(),
            len: self.len,
            _marker: marker::PhantomData,
        }
    }

    #[inline]
    pub fn overlaps<'a>(&'a self, k: &'a K) -> Overlaps<'a, K, V> {
        unsafe {
            Overlaps {
                k,
                node: self.root.min_overlapping(k),
                _marker: Default::default(),
            }
        }
    }

    #[inline]
    pub fn overlaps_mut<'a>(&'a mut self, k: &'a K) -> OverlapsMut<'a, K, V> {
        unsafe {
            OverlapsMut {
                k,
                node: self.root.min_overlapping(k),
                _marker: Default::default(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RBTree;

    #[test]
    fn test_insert() {
        let mut m = RBTree::new();
        assert_eq!(m.len(), 0);
        m.insert(1, 1, 2);
        assert_eq!(m.len(), 1);
        m.insert(2, 2, 4);
        m.insert(2, 20, 4);
        assert_eq!(m.len(), 3);
        assert_eq!(*m.get(&1, &1).unwrap(), 2);
        assert_eq!(*m.get(&2, &2).unwrap(), 4);
        assert_eq!(*m.get(&2, &2).unwrap(), 4);
        m.insert(2, 300, 6);
        m.insert(3, 232, 6);
        m.insert(5, 233, 6);
        m.insert(6, 233, 6);
        m.insert(7, 233, 6);
        m.insert(80, 232, 6);
        m.insert(250, 260, 6);
        m.insert(260, 270, 6);
        m.insert(270, 280, 6);
        m.insert(280, 290, 6);
        m.insert(0, 10000, 6);
        m.print_tree();
    }

    // #[test]
    // fn test_overlaps() {
    //     let mut m = RBTree::new();
    //     assert_eq!(m.len(), 0);
    //     m.do_insert(1, 1, 2);
    //     assert_eq!(m.len(), 1);
    //     m.do_insert(2, 2, 4);
    //     assert_eq!(m.len(), 2);
    //     m.do_insert(2, 2, 6);
    //     assert_eq!(m.len(), 3);
    //     assert_eq!(*m.get(&1, &1).unwrap(), 2);
    //     assert_eq!(*m.get(&2, &2).unwrap(), 4);
    //     assert_eq!(*m.get(&2, &2).unwrap(), 4);
    //     m.do_insert(2, 300, 6);
    //     m.do_insert(2, 232, 6);
    //     assert_eq!(Some((&(2, 232), &6)), m.overlaps(&30).next());
    //     assert_eq!(None, m.overlaps(&301).next());
    //     m.print_tree();
    // }

    #[cfg(test)]
    #[quickcheck_macros::quickcheck]
    fn test_overlaps(point: u16, items: Vec<(u16, u16)>) -> bool {
        use std::collections::HashSet;

        let tree = RBTree::from_iter(items.clone().into_iter().map(|v| (v.0, v.1, 0)));
        let laps = tree.overlaps(&point).collect::<Vec<_>>();
        let set = laps
            .clone()
            .into_iter()
            .map(|v| v.0)
            .collect::<HashSet<_>>();

        // println!("Tested: {} {:?}", point, items);
        // If range overlaps with point, it must be included, if it does not , it can't be
        items
            .iter()
            .all(|(s, e)| set.contains(&(*s, *e)) == (*s <= point && *e >= point))
    }

    // #[test]
    // fn test_replace() {
    //     let mut m = RBTree::new();
    //     assert_eq!(m.len(), 0);
    //     m.insert(2, 4);
    //     assert_eq!(m.len(), 1);
    //     assert_eq!(m.replace_or_insert(2, 6).unwrap(), 4);
    //     assert_eq!(m.len(), 1);
    //     assert_eq!(*m.get(&2).unwrap(), 6);
    // }
    //
    //
    // #[test]
    // fn test_clone() {
    //     let mut m = RBTree::new();
    //     assert_eq!(m.len(), 0);
    //     m.insert(1, 2);
    //     assert_eq!(m.len(), 1);
    //     m.insert(2, 4);
    //     assert_eq!(m.len(), 2);
    //     let m2 = m.clone();
    //     m.clear();
    //     assert_eq!(*m2.get(&1).unwrap(), 2);
    //     assert_eq!(*m2.get(&2).unwrap(), 4);
    //     assert_eq!(m2.len(), 2);
    // }
    //
    // #[test]
    // fn test_empty_remove() {
    //     let mut m: RBTree<isize, bool> = RBTree::new();
    //     assert_eq!(m.remove(&0), None);
    // }
    //
    // #[test]
    // fn test_empty_iter() {
    //     let mut m: RBTree<isize, bool> = RBTree::new();
    //     assert_eq!(m.iter().next(), None);
    //     assert_eq!(m.iter_mut().next(), None);
    //     assert_eq!(m.len(), 0);
    //     assert!(m.is_empty());
    //     assert_eq!(m.into_iter().next(), None);
    // }
    //
    // #[test]
    // fn test_lots_of_insertions() {
    //     let mut m = RBTree::new();
    //
    //     // Try this a few times to make sure we never screw up the hashmap's
    //     // internal state.
    //     for _ in 0..10 {
    //         assert!(m.is_empty());
    //
    //         for i in 1..101 {
    //             m.insert(i, i);
    //
    //             for j in 1..i + 1 {
    //                 let r = m.get(&j);
    //                 assert_eq!(r, Some(&j));
    //             }
    //
    //             for j in i + 1..101 {
    //                 let r = m.get(&j);
    //                 assert_eq!(r, None);
    //             }
    //         }
    //
    //         for i in 101..201 {
    //             assert!(!m.contains_key(&i));
    //         }
    //
    //         // remove forwards
    //         for i in 1..101 {
    //             assert!(m.remove(&i).is_some());
    //
    //             for j in 1..i + 1 {
    //                 assert!(!m.contains_key(&j));
    //             }
    //
    //             for j in i + 1..101 {
    //                 assert!(m.contains_key(&j));
    //             }
    //         }
    //
    //         for i in 1..101 {
    //             assert!(!m.contains_key(&i));
    //         }
    //
    //         for i in 1..101 {
    //             m.insert(i, i);
    //         }
    //
    //         // remove backwards
    //         for i in (1..101).rev() {
    //             assert!(m.remove(&i).is_some());
    //
    //             for j in i..101 {
    //                 assert!(!m.contains_key(&j));
    //             }
    //
    //             for j in 1..i {
    //                 assert!(m.contains_key(&j));
    //             }
    //         }
    //     }
    // }
    //
    // #[test]
    // fn test_find_mut() {
    //     let mut m = RBTree::new();
    //     m.insert(1, 12);
    //     m.insert(2, 8);
    //     m.insert(5, 14);
    //     let new = 100;
    //     match m.get_mut(&5) {
    //         None => panic!(),
    //         Some(x) => *x = new,
    //     }
    //     assert_eq!(m.get(&5), Some(&new));
    // }
    //
    // #[test]
    // fn test_remove() {
    //     let mut m = RBTree::new();
    //     m.insert(1, 2);
    //     assert_eq!(*m.get(&1).unwrap(), 2);
    //     m.insert(5, 3);
    //     assert_eq!(*m.get(&5).unwrap(), 3);
    //     m.insert(9, 4);
    //     assert_eq!(*m.get(&1).unwrap(), 2);
    //     assert_eq!(*m.get(&5).unwrap(), 3);
    //     assert_eq!(*m.get(&9).unwrap(), 4);
    //     assert_eq!(m.remove(&1).unwrap(), 2);
    //     assert_eq!(m.remove(&5).unwrap(), 3);
    //     assert_eq!(m.remove(&9).unwrap(), 4);
    //     assert_eq!(m.len(), 0);
    // }
    //
    // #[test]
    // fn test_is_empty() {
    //     let mut m = RBTree::new();
    //     m.insert(1, 2);
    //     assert!(!m.is_empty());
    //     assert!(m.remove(&1).is_some());
    //     assert!(m.is_empty());
    // }
    //
    // #[test]
    // fn test_pop() {
    //     let mut m = RBTree::new();
    //     m.insert(2, 4);
    //     m.insert(1, 2);
    //     m.insert(3, 6);
    //     assert_eq!(m.len(), 3);
    //     assert_eq!(m.pop_first(), Some((1, 2)));
    //     assert_eq!(m.len(), 2);
    //     assert_eq!(m.pop_last(), Some((3, 6)));
    //     assert_eq!(m.len(), 1);
    //     assert_eq!(m.get_first(), Some((&2, &4)));
    //     assert_eq!(m.get_last(), Some((&2, &4)));
    // }
    //
    // #[test]
    // fn test_iterate() {
    //     let mut m = RBTree::new();
    //     for i in 0..32 {
    //         m.insert(i, i * 2);
    //     }
    //     assert_eq!(m.len(), 32);
    //
    //     let mut observed: u32 = 0;
    //
    //     for (k, v) in m.iter() {
    //         assert_eq!(*v, *k * 2);
    //         observed |= 1 << *k;
    //     }
    //     assert_eq!(observed, 0xFFFF_FFFF);
    // }
    //
    // #[test]
    // fn test_keys() {
    //     let vec = vec![(1, 'a'), (2, 'b'), (3, 'c')];
    //     let map: RBTree<_, _> = vec.into_iter().collect();
    //     let keys: Vec<_> = map.keys().cloned().collect();
    //     assert_eq!(keys.len(), 3);
    //     assert!(keys.contains(&1));
    //     assert!(keys.contains(&2));
    //     assert!(keys.contains(&3));
    // }
    //
    // #[test]
    // fn test_values() {
    //     let vec = vec![(1, 'a'), (2, 'b'), (3, 'c')];
    //     let map: RBTree<_, _> = vec.into_iter().collect();
    //     let values: Vec<_> = map.values().cloned().collect();
    //     assert_eq!(values.len(), 3);
    //     assert!(values.contains(&'a'));
    //     assert!(values.contains(&'b'));
    //     assert!(values.contains(&'c'));
    // }
    //
    // #[test]
    // fn test_values_mut() {
    //     let vec = vec![(1, 1), (2, 2), (3, 3)];
    //     let mut map: RBTree<_, _> = vec.into_iter().collect();
    //     for value in map.values_mut() {
    //         *value = (*value) * 2
    //     }
    //     let values: Vec<_> = map.values().cloned().collect();
    //     assert_eq!(values.len(), 3);
    //     assert!(values.contains(&2));
    //     assert!(values.contains(&4));
    //     assert!(values.contains(&6));
    // }
    //
    // #[test]
    // fn test_find() {
    //     let mut m = RBTree::new();
    //     assert!(m.get(&1).is_none());
    //     m.insert(1, 2);
    //     match m.get(&1) {
    //         None => panic!(),
    //         Some(v) => assert_eq!(*v, 2),
    //     }
    // }
    //
    // #[test]
    // fn test_eq() {
    //     let mut m1 = RBTree::new();
    //     m1.insert(1, 2);
    //     m1.insert(2, 3);
    //     m1.insert(3, 4);
    //
    //     let mut m2 = RBTree::new();
    //     m2.insert(1, 2);
    //     m2.insert(2, 3);
    //
    //     assert!(m1 != m2);
    //
    //     m2.insert(3, 4);
    //
    //     assert_eq!(m1, m2);
    // }
    //
    // #[test]
    // fn test_show() {
    //     let mut map = RBTree::new();
    //     let empty: RBTree<i32, i32> = RBTree::new();
    //
    //     map.insert(1, 2);
    //     map.insert(3, 4);
    //
    //     let map_str = format!("{:?}", map);
    //
    //     assert!(map_str == "{1: 2, 3: 4}" || map_str == "{3: 4, 1: 2}");
    //     assert_eq!(format!("{:?}", empty), "{}");
    // }
    //
    // #[test]
    // fn test_from_iter() {
    //     let xs = [(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)];
    //
    //     let map: RBTree<_, _> = xs.iter().cloned().collect();
    //
    //     for &(k, v) in &xs {
    //         assert_eq!(map.get(&k), Some(&v));
    //     }
    // }
    //
    // #[test]
    // fn test_size_hint() {
    //     let xs = [(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)];
    //
    //     let map: RBTree<_, _> = xs.iter().cloned().collect();
    //
    //     let mut iter = map.iter();
    //
    //     for _ in iter.by_ref().take(3) {}
    //
    //     assert_eq!(iter.size_hint(), (3, Some(3)));
    // }
    //
    // #[test]
    // fn test_iter_len() {
    //     let xs = [(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)];
    //
    //     let map: RBTree<_, _> = xs.iter().cloned().collect();
    //
    //     let mut iter = map.iter();
    //
    //     for _ in iter.by_ref().take(3) {}
    //
    //     assert_eq!(iter.count(), 3);
    // }
    //
    // #[test]
    // fn test_mut_size_hint() {
    //     let xs = [(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)];
    //
    //     let mut map: RBTree<_, _> = xs.iter().cloned().collect();
    //
    //     let mut iter = map.iter_mut();
    //
    //     for _ in iter.by_ref().take(3) {}
    //
    //     assert_eq!(iter.size_hint(), (3, Some(3)));
    // }
    //
    // #[test]
    // fn test_iter_mut_len() {
    //     let xs = [(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)];
    //
    //     let mut map: RBTree<_, _> = xs.iter().cloned().collect();
    //
    //     let mut iter = map.iter_mut();
    //
    //     for _ in iter.by_ref().take(3) {}
    //
    //     assert_eq!(iter.count(), 3);
    // }
    //
    // #[test]
    // fn test_index() {
    //     let mut map = RBTree::new();
    //
    //     map.insert(1, 2);
    //     map.insert(2, 1);
    //     map.insert(3, 4);
    //
    //     assert_eq!(map[&2], 1);
    // }
    //
    // #[test]
    // #[should_panic]
    // fn test_index_nonexistent() {
    //     let mut map = RBTree::new();
    //
    //     map.insert(1, 2);
    //     map.insert(2, 1);
    //     map.insert(3, 4);
    //
    //     map[&4];
    // }
    //
    // #[test]
    // fn test_extend_iter() {
    //     let mut a = RBTree::new();
    //     a.insert(1, "one");
    //     let mut b = RBTree::new();
    //     b.insert(2, "two");
    //     b.insert(3, "three");
    //
    //     a.extend(b.into_iter());
    //
    //     assert_eq!(a.len(), 3);
    //     assert_eq!(a[&1], "one");
    //     assert_eq!(a[&2], "two");
    //     assert_eq!(a[&3], "three");
    // }
}
