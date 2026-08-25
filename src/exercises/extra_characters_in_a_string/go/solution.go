type TrieNode struct {
	children map[rune]*TrieNode
	isWord   bool
}

type Trie struct {
	root *TrieNode
}

func newTrieNode() *TrieNode {
	return &TrieNode{
		children: make(map[rune]*TrieNode),
		isWord:   false,
	}
}

func newTrie() *Trie {
	return &Trie{
		root: newTrieNode(),
	}
}

func (t *Trie) addWord(word string) {
	curr := t.root
	for _, c := range word {
		if _, exists := curr.children[c]; !exists {
			curr.children[c] = newTrieNode()
		}
		curr = curr.children[c]
	}
	curr.isWord = true
}

func minExtraChar(s string, dictionary []string) int {
	trie := newTrie()
	for _, w := range dictionary {
		trie.addWord(w)
	}

	n := len(s)
	dp := make([]int, n+1)

	for i := n - 1; i >= 0; i-- {
		dp[i] = 1 + dp[i+1]
		curr := trie.root

		for j := i; j < n; j++ {
			c := rune(s[j])
			if _, exists := curr.children[c]; !exists {
				break
			}
			curr = curr.children[c]
			if curr.isWord {
				dp[i] = min(dp[i], dp[j+1])
			}
		}
	}

	return dp[0]
}
