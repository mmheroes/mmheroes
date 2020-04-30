extension String {

    /// The difference between this and `split(separator:)` is that this property
    /// preserves newlines in the resulting strings.
    var lines: [(Substring, endsWithNewLine: Bool)] {
        let endIndex = self.endIndex
        var result = [(Substring, endsWithNewLine: Bool)]()
        var substringStart = startIndex
        var i = substringStart
        while true {
            if i == endIndex {
                result.append((self[substringStart ..< i], false))
                break
            }
            if self[i] == "\n" {
                result.append((self[substringStart ..< i], true))
                substringStart = index(after: i)
                i = substringStart
                continue
            }
            formIndex(after: &i)
        }
        return result
    }
}
