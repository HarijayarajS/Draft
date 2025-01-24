
# Markdown Syntax Cheat Sheet

This file demonstrates all types of Markdown syntax that you can use for your documentation.

---

## 1. Headers

```markdown
# H1 Header
## H2 Header
### H3 Header
#### H4 Header
##### H5 Header
###### H6 Header
```

---

## 2. Emphasis

```markdown
*Italic text* or _Italic text_  
**Bold text** or __Bold text__  
***Bold and Italic text***  
~~Strikethrough~~
```

---

## 3. Lists

### Unordered List
```markdown
- Item 1
  - Sub-item 1.1
  - Sub-item 1.2
- Item 2
```

### Ordered List
```markdown
1. First item
2. Second item
   1. Sub-item 2.1
   2. Sub-item 2.2
3. Third item
```

---

## 4. Links

```markdown
[Inline Link](https://example.com)  
[Reference Link][1]  

[1]: https://example.com "Example Website"
```

---

## 5. Images

```markdown
![Alt text](https://via.placeholder.com/150 "Optional Title")
```

---

## 6. Code Blocks

### Inline Code
```markdown
`Inline code`
```

### Code Block
```markdown
```rust
fn main() {
    println!("Hello, world!");
}
```
```

---

## 7. Blockquotes

```markdown
> This is a blockquote.
>> Nested blockquote.
```

---

## 8. Tables

```markdown
| Header 1 | Header 2 | Header 3 |
|----------|----------|----------|
| Row 1    | Data     | Data     |
| Row 2    | Data     | Data     |
```

---

## 9. Horizontal Rules

```markdown
---
```

---

## 10. Task Lists

```markdown
- [x] Completed task
- [ ] Incomplete task
```

---

## 11. HTML in Markdown

```markdown
<div style="color: blue;">This text is blue.</div>
```

---

## 12. Escaping Special Characters

```markdown
Use a backslash to escape special characters: \* \_ \` \[
```