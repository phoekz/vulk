#[derive(Clone, Copy, Debug)]
pub(super) struct Node<'a>(roxmltree::Node<'a, 'a>);

impl<'a> Node<'a> {
    pub(super) fn from(node: roxmltree::Node<'a, 'a>) -> Self {
        Self(node)
    }

    pub(super) fn tag(self) -> &'a str {
        self.0.tag_name().name()
    }

    pub(super) fn attribute(self, attr_name: &str) -> Option<String> {
        self.0.attribute(attr_name).map(ToOwned::to_owned)
    }

    pub(super) fn required_attribute(self, attr_name: &str) -> String {
        self.attribute(attr_name)
            .unwrap_or_else(|| panic!("{attr_name} is required"))
    }

    pub(super) fn child(self, tag_name: &str) -> Option<Node<'a>> {
        self.0.children().find_map(|node| {
            if node.tag_name().name() == tag_name {
                Some(Self(node))
            } else {
                None
            }
        })
    }

    pub(super) fn child_text(self, tag_name: &str) -> Option<String> {
        self.child(tag_name)?.0.text().map(ToOwned::to_owned)
    }

    pub(super) fn required_child_text(self, tag_name: &str) -> String {
        self.child_text(tag_name)
            .unwrap_or_else(|| panic!("{tag_name} is required"))
    }

    pub(super) fn children(self, tag_name: &'a str) -> impl Iterator<Item = Node<'a>> {
        self.0.children().filter_map(move |node| {
            if node.tag_name().name() == tag_name {
                Some(Self(node))
            } else {
                None
            }
        })
    }

    pub(super) fn children_elements(self, tag_name: &'a str) -> impl Iterator<Item = Node<'a>> {
        self.children(tag_name).filter(|node| node.0.is_element())
    }

    pub(super) fn children_any_elements(self) -> impl Iterator<Item = Node<'a>> {
        self.0
            .children()
            .filter(roxmltree::Node::is_element)
            .map(Self)
    }

    pub(super) fn children_under_parent(
        self,
        parent_tag_name: &str,
        tag_name: &'a str,
    ) -> impl Iterator<Item = Node<'a>> {
        let parent_node = self
            .child(parent_tag_name)
            .unwrap_or_else(|| panic!("{parent_tag_name} is required"));
        parent_node.children(tag_name)
    }

    pub(super) fn children_text(self, tag_name: &'a str) -> impl Iterator<Item = String> + '_ {
        self.0
            .children()
            .filter(roxmltree::Node::is_element)
            .filter(move |node| node.tag_name().name() == tag_name)
            .map(|node| Self(node).joined_children_text().unwrap())
    }

    pub(super) fn joined_children_text(self) -> Option<String> {
        let binding = self
            .0
            .children()
            .filter(roxmltree::Node::is_text)
            .filter_map(|node| node.text())
            .collect::<String>();
        let str = binding.trim();
        let str = str
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();
        if str.is_empty() {
            return None;
        }
        Some(str)
    }
}
