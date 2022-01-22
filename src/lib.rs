use std::mem;
use std::collections::BTreeMap;
use either::Either;

pub fn merge_consecutive<K, V, P, M>(map: &mut BTreeMap<K, V>, mut predicate: P, mut absorb: M)
    where
        K: Ord,
        P: FnMut(&(K, V), &(K, V)) -> bool,
        M: FnMut((&K, &mut V), (&K, &mut V)),
{
    if map.len() < 2 {
        return;
    }

    let mut iter = mem::take(map).into_iter();
    let mut curr = vec![iter.next().unwrap()];
    let mut next = iter.next().unwrap();

    loop {
        let curr_kv = curr.last_mut().unwrap();
        if predicate(curr_kv, &next) {
            absorb((&curr_kv.0, &mut curr_kv.1), (&next.0, &mut next.1));
        } else {
            curr.push(next);
        }
        match iter.next() {
            Some(kv) => next = kv,
            None => break
        }
    }

    *map = curr.into_iter().collect();
}

pub fn merge<K, V, P, M>(map: &mut BTreeMap<K, V>, mut predicate: P, mut absorb: M)
    where
        K: Clone + Ord,
        P: FnMut((&K, &V), (&K, &V)) -> bool,
        M: FnMut((&K, &mut V), (&K, V)),
{
    let mut merge_list: Vec<(K, K)> = Vec::new();
    let mut current = Either::Left(map.iter()).peekable();
    let mut next = current.clone().skip(1);

    loop {
        match (current.peek(), next.next()) {
            (Some(&curr_kv), Some(next_kv)) => {
                if predicate(curr_kv, next_kv) {
                    merge_list.push(((*curr_kv.0).clone(), (*next_kv.0).clone()));
                } else {
                    current = Either::Right(map.range((*next_kv.0).clone()..)).peekable();
                }
            }
            _ => break,
        }
    }

    for (dest, src) in merge_list {
        let src_element = map.remove(&src).expect("Not found (src)");
        let dst_element = map.get_mut(&dest).expect("Not found (get_mut)");
        absorb((&dest, dst_element), (&src, src_element));
    }
}
