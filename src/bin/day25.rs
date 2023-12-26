use graphrs::algorithms::community::louvain::louvain_partitions;
use graphrs::{Edge, Graph, GraphSpecs};

const INPUT: &str = "./input/day25.txt";

fn part1(input: &str) -> usize {
    let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::undirected_create_missing());
    for line in input.lines() {
        let (component, others) = line.split_once(':').unwrap();
        for other in others.split_whitespace() {
            graph.add_edge(Edge::new(component, other)).unwrap();
        }
    }

    let res = louvain_partitions(&graph, false, Some(0.0), None, None).unwrap();
    res[0].iter().map(|partition| partition.len()).product()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;

    println!("The first answer is: {}", part1(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        jqt: rhn xhk nvd\n\
        rsh: frs pzl lsr\n\
        xhk: hfx\n\
        cmg: qnr nvd lhk bvb\n\
        rhn: xhk bvb hfx\n\
        bvb: xhk hfx\n\
        pzl: lsr hfx nvd\n\
        qnr: nvd\n\
        ntq: jqt hfx bvb xhk\n\
        nvd: lhk\n\
        lsr: lhk\n\
        rzs: qnr cmg lsr rsh\n\
        frs: qnr lhk lsr\n\
    ";

    #[test]
    fn test_part1() {
        let actual = part1(EXAMPLE);
        let expected = 54;

        assert_eq!(expected, actual);
    }
}
