use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone)]
pub struct Arena<T> {
    players: HashMap<T, (usize, usize)>,
}

impl<T> Arena<T>
where
    T: Hash + Eq,
{
    pub fn new() -> Self {
        Arena {
            players: HashMap::new(),
        }
    }
}

impl<T> Arena<T>
where
    T: Hash + Eq,
{
    pub fn insert(&mut self, id: T, rank: usize) -> Option<(usize, usize)> {
        self.players.insert(id, (rank, rank))
    }

    pub fn remove<Q>(&mut self, id: &Q) -> Option<(usize, usize)>
    where
        T: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.players.remove(id)
    }
}

impl<T> Arena<T>
where
    T: Hash + Eq,
{
    pub fn rank_update(&mut self) {
        for (_, (min_rank_i, max_rank_i)) in &mut self.players {
            if *min_rank_i > usize::min_value() {
                *min_rank_i -= 1;
            }
            if *max_rank_i < usize::max_value() {
                *max_rank_i += 1;
            }
        }
    }

    pub fn rank_match(&self) -> Vec<&T> {
        let mut max_rank = usize::min_value();
        let mut min_rank = usize::max_value();
        for (_, (min_rank_i, max_rank_i)) in &self.players {
            max_rank = usize::max(max_rank, *max_rank_i);
            min_rank = usize::min(min_rank, *min_rank_i);
        }
        let mut cnt = vec![0isize; max_rank - min_rank + 2];
        for (_, (min_rank_i, max_rank_i)) in &self.players {
            let index_l = min_rank_i - min_rank;
            let index_r = max_rank_i - min_rank + 1;
            cnt[index_l] += 1;
            cnt[index_r] -= 1;
        }
        let mut max_cnt = isize::min_value();
        let mut max_cnt_i = 0;
        for i in 1..cnt.len() {
            cnt[i] += cnt[i - 1];
            if cnt[i] > max_cnt {
                max_cnt_i = i;
                max_cnt = cnt[i];
            }
        }
        let target_rank = max_cnt_i + min_rank;
        let mut ans = Vec::new();
        for (id, (min_rank_i, max_rank_i)) in &self.players {
            if *min_rank_i <= target_rank && target_rank <= *max_rank_i {
                ans.push(id)
            }
        }
        ans
    }
}

fn main() {
    let mut arena = Arena::new();
    arena.insert("boybook", 100);
    arena.insert("luojia65", 101);
    // 第0秒
    let ans = arena.rank_match();
    println!("{:?}", ans);
    // 每秒钟结束时更新
    arena.rank_update();
    // 第1秒
    let ans = arena.rank_match();
    println!("{:?}", ans);
}
