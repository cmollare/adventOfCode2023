use regex::Regex;

#[derive(Debug)]
#[derive(Clone)]
struct Galaxy {
    x: usize,
    y: usize,
}

#[derive(Debug)]
#[derive(Clone)]
struct Pair {
    left: Galaxy,
    right: Galaxy
}

pub fn day11_part2(t: &str) {
    let galaxies: Vec<_> = t.lines().enumerate().flat_map(|(i, l)|parse_galaxy(l, i)).collect();
    let galaxies = expand_univers(&galaxies);
    let pairs = pair_galaxies(&galaxies);
    let res: usize = pairs.iter().map(compute_shortest_path).sum();

    println!("galaxies {:?}", galaxies);
    println!("nb pairs {:?}", pairs.len());
    println!("result {}", res);

}

fn compute_shortest_path(p: &Pair) -> usize {
    let distance = compute_distance(&p.left, &p.right);
    //println!("distances {:?} {:?} {:?}", distance, p.left, p.right);
    return distance;
}

fn compute_distance(g1: &Galaxy, g2: &Galaxy) -> usize {
    let x = if g1.x > g2.x { g1.x - g2.x } else { g2.x - g1.x };
    let y = if g1.y > g2.y { g1.y - g2.y } else { g2.y - g1.y };
    let res = x+y;
    return res;
}

fn pair_galaxies(g: &Vec<Galaxy>) -> Vec<Pair> {
    let mut pairs = Vec::new();

    for i in 0..g.len() {
        let cur_galaxy = &g[i];
        for j in (i+1)..g.len() {
            pairs.push(Pair{
                left: cur_galaxy.clone(),
                right: g[j].clone()
            })
        }
    }

    return pairs;
}

fn expand_univers(g: &Vec<Galaxy>) -> Vec<Galaxy> {
    let max_l = g.iter().map(|g| g.x).max().unwrap();
    let max_c = g.iter().map(|g| g.y).max().unwrap();
    let mut res = g.clone();
    let step = 999999;
    let mut offset = 0;

    for i in 0..max_l {
        let pos = i + offset;
        if g.iter().all(|g| g.x != i) {
            res = shift_galaxy_line(&res, pos, step);
            offset = offset + step;
        }
    }

    offset = 0;
    for i in 0..max_c {
        let pos = i + offset;
        if g.iter().all(|g| g.y != i) {
            res = shift_galaxy_col(&res, pos, step);
            offset = offset + step;
        }
    }

    return res;
}

fn shift_galaxy_line(g: &Vec<Galaxy>, l: usize, step: usize) -> Vec<Galaxy> {
    return g.iter().map(|g| if g.x > l { Galaxy { x: g.x+step, y: g.y }} else { g.clone() }).collect();
}

fn shift_galaxy_col(g: &Vec<Galaxy>, c: usize, step: usize) -> Vec<Galaxy> {
    return g.iter().map(|g| if g.y > c { Galaxy { x: g.x, y: g.y+step }} else { g.clone() }).collect();
}

fn parse_galaxy(l: &str, cur_l: usize) -> Vec<Galaxy> {
    let re = Regex::new(r"(#)").unwrap();
    let res = re.find_iter(l).map(|c| Galaxy{ x: cur_l,y: c.start()}).collect();

    return res;
}