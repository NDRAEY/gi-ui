pub enum VerticalAlignment {
	TOP,
	CENTER,
	BOTTOM
}

pub enum HorizontalAlignment {
	LEFT,
	CENTER,
	RIGHT
}

pub trait VAlignment {
	fn set_vertical_alignment(&mut self, align: VerticalAlignment);
}
