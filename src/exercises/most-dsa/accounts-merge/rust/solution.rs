impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let n = accounts.len();
        let mut uf = UnionFind::new(n);
        let mut email_to_acc: HashMap<String, usize> = HashMap::new();

        // Build union-find structure
        for i in 0..n {
            for j in 1..accounts[i].len() {
                let email = &accounts[i][j];
                if let Some(&acc_id) = email_to_acc.get(email) {
                    uf.union(i, acc_id);
                } else {
                    email_to_acc.insert(email.clone(), i);
                }
            }
        }

        // Group emails by leader account
        let mut email_group: HashMap<usize, Vec<String>> = HashMap::new();
        for (email, acc_id) in email_to_acc.iter() {
            let leader = uf.find(*acc_id);
            email_group.entry(leader).or_insert_with(Vec::new).push(email.clone());
        }

        // Build result
        let mut res: Vec<Vec<String>> = Vec::new();
        for (acc_id, mut emails) in email_group {
            emails.sort();
            let mut merged = vec![accounts[acc_id][0].clone()];
            merged.extend(emails);
            res.push(merged);
        }

        res
    }
}
