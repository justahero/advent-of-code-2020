# Day 19

[Advent Of Code Day 19](https://adventofcode.com/2020/day/19).

## Part 1

Applying rules to validate messages.

* list of rules and messages
* rules:
  * some rules match a single character: `3: "b"` matches single character `b`
  * rule `0: 1 2` means that to match rule `0` the text must match rule `1` and the text after the part that matched rule `1` must then match rule `2`

Example

```
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"
```

* rule `4` matches `a` and rule `5` matches `b`
* rule `2` matches two letters that are the same (`aa` or `bb`)
* rule `3` matches two letters that are different (`ab` or `ba`)
* since rule `1` matches rules `2` and `3` once each in either order, it must match two pairs of letters
  * for example: `aabb`, `aaba`, `bbab`, `bbba`, `aabb`, `abaa`, `abbb`, `baaa`, `babb`
* rule `0` therefore matches `a` (rule `4`), then any of the eight options from rule `1`, then `b` (rule `5`)
  * matches: `aaaabb`, `aaabab`, `abbabb`, `abbbab`, `aabaab`, `aabbbb`, `abaaab`, or `ababbb`

Received messages need to be checked against the rules

```
ababbb
bababa
abbbab
aaabbb
aaaabbb
```

In the above example, `ababbb` and `abbbab` match, but `bababa`, `aaabbb`, and `aaaabbb` do not.