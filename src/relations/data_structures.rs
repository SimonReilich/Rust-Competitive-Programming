pub struct UnionFind {
    source: Vec<usize>,
    size: Vec<usize>
}

impl UnionFind {
    pub fn new () -> UnionFind {
        return UnionFind { source: Vec::new(), size: Vec::new() };
    }

    pub fn union (self: &mut Self, a: usize, b: usize) -> usize {
        let source_a = self.find(a);
        let source_b = self.find(b);
        if source_a == source_b {
            return source_a;
        } else if self.size(source_a) > self.size(source_b) {
            self.create(source_a);
            self.create(source_b);
            self.source[source_b] = source_a;
            self.size[source_a] += self.size[source_b];
            return source_a;
        } else {
            self.create(source_a);
            self.create(source_b);
            self.source[source_a] = source_b;
            self.size[source_b] += self.size[source_a];
            return source_b;
        }
    }

    pub fn find (self: &mut Self, a: usize) -> usize {
        if a >= self.source.len() || a == self.source[a] {
            return a;
        } else {
            let source = self.find(self.source[a]);
            self.source[a] = source;
            return source;
        }
    }

    fn create (self: &mut Self, a: usize) {
        while self.source.len() <= a {
            self.source.push(self.source.len());
            self.size.push(1);
        }
    }

    fn size (self: &mut Self, a: usize) -> usize {
        if a >= self.size.len() {
            return 1;
        } else {
            let source = self.find(a);
            return self.size[source];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let uf = UnionFind::new();
        assert_eq!(uf.source.len(), 0, "Source vector should be empty initially");
        assert_eq!(uf.size.len(), 0, "Size vector should be empty initially");
    }

    #[test]
    fn test_find_new_elements() {
        let mut uf = UnionFind::new();
        // Finding a new element should initialize it, and it should be its own root.
        assert_eq!(uf.find(0), 0, "Element 0 should be its own root.");
        assert_eq!(uf.find(5), 5, "Element 5 should be its own root.");
        // Check internal size of the set (which is 1 for a new element)
        assert_eq!(uf.size(0), 1, "Set containing 0 should have size 1.");
        assert_eq!(uf.size(5), 1, "Set containing 5 should have size 1.");
    }

    #[test]
    fn test_simple_union() {
        let mut uf = UnionFind::new();

        // Union 1 and 2
        let root_1_2 = uf.union(1, 2);
        
        // Now 1 and 2 should have the same root
        assert_eq!(uf.find(1), root_1_2, "1 should link to the new root");
        assert_eq!(uf.find(2), root_1_2, "2 should link to the new root");
        assert_eq!(uf.find(1), uf.find(2), "1 and 2 must have the same root");
        
        // Set size must be 2
        assert_eq!(uf.size(root_1_2), 2, "Set size for {root_1_2} should be 2");
        
        // 3 should still be in its own set
        assert_ne!(uf.find(3), root_1_2, "3 should not be in the same set");
        assert_eq!(uf.size(3), 1, "Set size for 3 should be 1");
    }

    #[test]
    fn test_chained_union_and_path_compression() {
        let mut uf = UnionFind::new();

        // Chain elements: 1 -> 2 -> 3 (The final root depends on the union-by-size logic)
        uf.union(1, 2); 
        uf.union(2, 3);
        let root_1_2_3 = uf.find(1); // The final root

        // All should belong to the same set
        assert_eq!(uf.find(1), root_1_2_3, "1 must belong to the final root");
        assert_eq!(uf.find(2), root_1_2_3, "2 must belong to the final root");
        assert_eq!(uf.find(3), root_1_2_3, "3 must belong to the final root");
        
        // Set size must be 3
        assert_eq!(uf.size(root_1_2_3), 3, "Set size for {root_1_2_3} should be 3");

        // Verify path compression: After the find operations, 1, 2, and 3 should all point directly
        // to the root in the `source` vector (unless one of them is the root itself).
        assert_eq!(uf.source[1], root_1_2_3);
        assert_eq!(uf.source[2], root_1_2_3);
        assert_eq!(uf.source[3], root_1_2_3);
    }

    #[test]
    fn test_union_same_set() {
        let mut uf = UnionFind::new();

        let root_a = uf.union(10, 11);
        
        // Union them again - the root should remain the same and size shouldn't change
        let root_b = uf.union(10, 11); 
        
        assert_eq!(root_a, root_b, "Union of elements in the same set should return the same root");
        assert_eq!(uf.size(root_a), 2, "Set size should remain 2");

        // Union 10 and 12 (12 is new)
        let root_c = uf.union(10, 12);
        
        // The root should be either root_a or 12, but the final set size must be 3
        assert_eq!(uf.size(root_c), 3, "Set size should be 3 after adding 12");
        assert_eq!(uf.find(10), uf.find(11));
        assert_eq!(uf.find(11), uf.find(12));
    }

    #[test]
    fn test_disjoint_set_merging() {
        let mut uf = UnionFind::new();

        // Set A: {1, 2, 3}
        uf.union(1, 2);
        uf.union(2, 3);
        let root_a = uf.find(1);
        assert_eq!(uf.size(root_a), 3);

        // Set B: {10, 11}
        uf.union(10, 11);
        let root_b = uf.find(10);
        assert_eq!(uf.size(root_b), 2);
        
        assert_ne!(root_a, root_b, "The two sets must be disjoint initially");
        assert_ne!(uf.find(1), uf.find(10), "1 and 10 must have different roots");

        // Merge Set A and Set B by unioning 2 and 10
        let final_root = uf.union(2, 10);
        
        // Final set size must be 3 + 2 = 5
        assert_eq!(uf.size(final_root), 5, "Final set size should be 5");

        // All elements must share the final root
        assert_eq!(uf.find(1), final_root);
        assert_eq!(uf.find(3), final_root);
        assert_eq!(uf.find(11), final_root);
    }
}