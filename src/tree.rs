use std::{collections::VecDeque, fmt::{self, Display}, rc::Rc};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct SpacePalette {
    skip: &'static str,
    indent: &'static str,
}

type DisplayQueue<'t, D> = VecDeque<(bool, &'t Tree<D>, &'t GlyphPalette, Rc<Vec<SpacePalette>>)>;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct GlyphPalette {
    pub middle_item: &'static str,
    pub last_item: &'static str,
    pub item_indent: &'static str,

    pub middle_skip: &'static str,
    pub last_skip: &'static str,
    pub skip_indent: &'static str,
}

impl GlyphPalette {
    pub const fn new() -> Self {
        Self {
            middle_item: "├",
            last_item: "└",
            item_indent: "── ",

            middle_skip: "│",
            last_skip: " ",
            skip_indent: "   ",
        }
    }

    fn middle_space(&self) -> SpacePalette {
        SpacePalette {
            skip: self.middle_skip,
            indent: self.skip_indent,
        }
    }

    fn last_space(&self) -> SpacePalette {
        SpacePalette {
            skip: self.last_skip,
            indent: self.skip_indent,
        }
    }
}

impl Default for GlyphPalette {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Tree<D: Display> {
    pub root: D,
    pub leaves: Vec<Tree<D>>,
    multiline: bool,
    glyphs: Option<GlyphPalette>,
}

impl<D: Display> Tree<D> {
    pub fn new(root: D) -> Self {
        Tree {
            root,
            leaves: Vec::new(),
            multiline: false,
            glyphs: None,
        }
    }

    /// Ensure all lines for `root` are indented
    pub fn with_multiline(mut self, yes: bool) -> Self {
        self.multiline = yes;
        self
    }
}

impl<D: Display> Tree<D> {
    /// Ensure all lines for `root` are indented
    pub fn set_multiline(&mut self, yes: bool) -> &mut Self {
        self.multiline = yes;
        self
    }
}

impl<D: Display> Tree<D> {
    pub fn push(&mut self, leaf: impl Into<Tree<D>>) -> &mut Self {
        self.leaves.push(leaf.into());
        self
    }
}

fn enqueue_leaves<'t, D: Display>(
    queue: &mut DisplayQueue<'t, D>,
    parent: &'t Tree<D>,
    parent_glyphs: &'t GlyphPalette,
    spaces: Rc<Vec<SpacePalette>>,
) {
    for (i, leaf) in parent.leaves.iter().rev().enumerate() {
        let last = i == 0;
        let glyphs = leaf.glyphs.as_ref().unwrap_or(parent_glyphs);
        queue.push_front((last, leaf, glyphs, spaces.clone()));
    }
}

impl<D: Display> Display for Tree<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.root.fmt(f)?; // Pass along `f.alternate()`
        writeln!(f)?;
        let mut queue = DisplayQueue::new();
        let no_space = Rc::new(Vec::new());
        let default_glyphs = GlyphPalette::new();
        let glyphs = self.glyphs.as_ref().unwrap_or(&default_glyphs);
        enqueue_leaves(&mut queue, self, glyphs, no_space);
        while let Some((last, leaf, glyphs, spaces)) = queue.pop_front() {
            let mut prefix = (
                if last {
                    glyphs.last_item
                } else {
                    glyphs.middle_item
                },
                glyphs.item_indent,
            );

            if leaf.multiline {
                let rest_prefix = (
                    if last {
                        glyphs.last_skip
                    } else {
                        glyphs.middle_skip
                    },
                    glyphs.skip_indent,
                );
                debug_assert_eq!(prefix.0.chars().count(), rest_prefix.0.chars().count());
                debug_assert_eq!(prefix.1.chars().count(), rest_prefix.1.chars().count());

                let root = if f.alternate() {
                    format!("{:#}", leaf.root)
                } else {
                    format!("{:}", leaf.root)
                };
                for line in root.lines() {
                    // print single line
                    for s in spaces.as_slice() {
                        s.skip.fmt(f)?;
                        s.indent.fmt(f)?;
                    }
                    prefix.0.fmt(f)?;
                    prefix.1.fmt(f)?;
                    line.fmt(f)?;
                    writeln!(f)?;
                    prefix = rest_prefix;
                }
            } else {
                // print single line
                for s in spaces.as_slice() {
                    s.skip.fmt(f)?;
                    s.indent.fmt(f)?;
                }
                prefix.0.fmt(f)?;
                prefix.1.fmt(f)?;
                leaf.root.fmt(f)?; // Pass along `f.alternate()`
                writeln!(f)?;
            }

            // recurse
            if !leaf.leaves.is_empty() {
                let s: &Vec<SpacePalette> = &spaces;
                let mut child_spaces = s.clone();
                child_spaces.push(if last {
                    glyphs.last_space()
                } else {
                    glyphs.middle_space()
                });
                let child_spaces = Rc::new(child_spaces);
                enqueue_leaves(&mut queue, leaf, glyphs, child_spaces);
            }
        }
        Ok(())
    }
}