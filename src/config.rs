pub struct Resolution {
    pub width: usize,
    pub height: usize,
}

pub struct DesktopCastConfig {
    pub target_resolution: Option<Resolution>,
}
