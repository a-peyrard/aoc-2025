pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }
    }

    pub fn component_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_initialize_each_element_in_its_own_component() {
        // GIVEN
        let n = 5;

        // WHEN
        let mut uf = UnionFind::new(n);

        // THEN
        for i in 0..n {
            assert_eq!(uf.find(i), i);
            assert_eq!(uf.component_size(i), 1);
        }
    }

    #[test]
    fn test_should_connect_two_elements_after_union() {
        // GIVEN
        let mut uf = UnionFind::new(5);

        // WHEN
        uf.union(0, 1);

        // THEN
        assert_eq!(uf.find(0), uf.find(1));
        assert_eq!(uf.component_size(0), 2);
        assert_eq!(uf.component_size(1), 2);
    }

    #[test]
    fn test_should_create_correct_component_sizes_with_multiple_unions() {
        // GIVEN
        let mut uf = UnionFind::new(5);

        // WHEN
        uf.union(0, 1);
        uf.union(1, 2);
        uf.union(3, 4);

        // THEN
        assert_eq!(uf.find(0), uf.find(1));
        assert_eq!(uf.find(1), uf.find(2));
        assert_eq!(uf.find(3), uf.find(4));
        assert_ne!(uf.find(0), uf.find(3));
        assert_eq!(uf.component_size(0), 3);
        assert_eq!(uf.component_size(3), 2);
    }

    #[test]
    fn test_should_handle_union_of_already_connected_elements() {
        // GIVEN
        let mut uf = UnionFind::new(3);
        uf.union(0, 1);

        // WHEN
        uf.union(0, 1);

        // THEN
        assert_eq!(uf.component_size(0), 2);
    }

    #[test]
    fn test_should_merge_two_existing_components() {
        // GIVEN
        let mut uf = UnionFind::new(6);
        uf.union(0, 1);
        uf.union(2, 3);

        // WHEN
        uf.union(1, 2);

        // THEN
        assert_eq!(uf.find(0), uf.find(3));
        assert_eq!(uf.component_size(0), 4);
        assert_ne!(uf.find(0), uf.find(4));
    }
}
