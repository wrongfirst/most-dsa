func simplifyPath(path string) string {
	stack := []string{}
	paths := strings.Split(path, "/")

	for _, cur := range paths {
		if cur == ".." {
			if len(stack) > 0 {
				stack = stack[:len(stack)-1]
			}
		} else if cur != "" && cur != "." {
			stack = append(stack, cur)
		}
	}

	return "/" + strings.Join(stack, "/")
}
