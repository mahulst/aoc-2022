use std::collections::{BTreeMap, HashMap};

pub fn run_part_1(input: &str) -> String {
    let mut y = -1;
    let mut map: BTreeMap<(i32, i32), u32> = BTreeMap::new();
    let mut x = -1;
    input.lines().for_each(|l| {
        y += 1;
        x = -1;
        return l.chars().map(|c| c.to_digit(10).unwrap()).for_each(|c| {
            x += 1;

            map.insert((x, y), c.clone());
        });
    });
    let length = y;
    assert_eq!(x, y);
    let mut count = 0;
    for y in 1..length {
        for x in 1..length {
            if is_visible(&map, length as i32, (x, y), map.get(&(x, y)).unwrap()) {
                count += 1;
            }
        }
    }

    return (count + length * 4).to_string();
}

fn is_visible(trees: &BTreeMap<(i32, i32), u32>, length: i32, (tree_x, tree_y): (i32, i32), height: &u32) -> bool {
    // top
    let mut top_visible = true;
    for y in 0..tree_y {
        if !top_visible {
            break;
        }
        let cursor_height = trees.get(&(tree_x, y)).unwrap();
        if cursor_height >= height { top_visible = false }
    }
    // bottom
    let mut bottom_visible = true;
    for y in tree_y + 1..=length {
        if !bottom_visible {
            break;
        }
        let cursor_height = trees.get(&(tree_x, y)).unwrap();
        if cursor_height >= height { bottom_visible = false }
    }

    // right
    let mut right_visible = true;
    for x in tree_x + 1..=length {
        if !right_visible {
            break;
        }
        let cursor_height = trees.get(&(x, tree_y)).unwrap();
        if cursor_height >= height { right_visible = false }
    }
    // right

    let mut left_visible = true;
    for x in 0..tree_x {
        if !left_visible {
            break;
        }
        let cursor_height = trees.get(&(x, tree_y)).unwrap();
        if cursor_height >= height { left_visible = false }
    }
    top_visible || bottom_visible || left_visible || right_visible
}

pub fn run_part_2(input: &str) -> String {
    let mut y = -1;
    let mut map: BTreeMap<(i32, i32), u32> = BTreeMap::new();
    let mut x = -1;
    input.lines().for_each(|l| {
        y += 1;
        x = -1;
        return l.chars().map(|c| c.to_digit(10).unwrap()).for_each(|c| {
            x += 1;

            map.insert((x, y), c.clone());
        });
    });
    let length = y;
    assert_eq!(x, y);
    let mut count = 0;
    for y in 1..length {
        for x in 1..length {
            let new_count = get_scenic_score(&map, length as i32, (x, y), map.get(&(x, y)).unwrap());

            count = count.max(new_count);
        }
    }

    return count.to_string();
}


fn get_scenic_score(trees: &BTreeMap<(i32, i32), u32>, length: i32, (tree_x, tree_y): (i32, i32), height: &u32) -> u32 {
    // top
    let mut top_trees_visible = 0;
    let mut last_height = height;
    for y in (0..tree_y).rev() {
        top_trees_visible += 1;
        ;
        let cursor_height = trees.get(&(tree_x, y)).unwrap();
        if cursor_height >= last_height { break; } else { last_height = last_height.max(cursor_height) }
    }
    // bottom
    let mut bottom_trees_visible = 0;
    let mut last_height = height;
    for y in tree_y + 1..=length {
        bottom_trees_visible += 1;
        let cursor_height = trees.get(&(tree_x, y)).unwrap();
        if cursor_height >= last_height { break; } else { last_height = last_height.max(cursor_height) }
    }

    // right
    let mut right_trees_visible = 0;
    let mut last_height = height;
    for x in tree_x + 1..=length {
        right_trees_visible += 1;
        let cursor_height = trees.get(&(x, tree_y)).unwrap();
        if cursor_height >= last_height { break; } else { last_height = last_height.max(cursor_height) }
    }
    // right

    let mut left_trees_visible = 0;
    let mut last_height = height;
    for x in (0..tree_x).rev() {
        left_trees_visible += 1;
        let cursor_height = trees.get(&(x, tree_y)).unwrap();
        if cursor_height >= last_height { break; } else { last_height = last_height.max(cursor_height) }
    }

    top_trees_visible * bottom_trees_visible * left_trees_visible * right_trees_visible
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = run_part_1("30373
25512
65332
33549
35390");
        assert_eq!(result, "21");
    }

    #[test]
    fn it_works2() {
        let result = run_part_2("30373
25512
65332
33549
35390");
        assert_eq!(result, "8");
    }
}
