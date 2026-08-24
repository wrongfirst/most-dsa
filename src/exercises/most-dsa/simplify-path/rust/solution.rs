impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack: Vec<&str> = Vec::new();
        let paths: Vec<&str> = path.split('/').collect();

        for cur in paths {
            if cur == ".." {
                stack.pop();
            } else if !cur.is_empty() && cur != "." {
                stack.push(cur);
            }
        }

        format!("/{}", stack.join("/"))
    }
}
