#[derive(Debug, Clone)]
pub struct BoxDim {
    pub width: u32,
    pub depth: u32,
    pub height: u32,
}

impl BoxDim {
    pub fn can_be_above(&self, other: &BoxDim) -> bool {
        self.width < other.width && self.depth < other.depth && self.height < other.height
    }
}

/// 与えられた箱リストから最大積み上げ高さを計算する
pub fn max_stack_height(boxes: &[BoxDim]) -> u32 {
    let mut sorted_boxes = boxes.to_vec();
    // 高さ降順でソート（スタックの下から順に処理したい）
    sorted_boxes.sort_by(|a, b| b.height.cmp(&a.height));

    let mut memo = vec![None; sorted_boxes.len()];
    max_height_recursive(&sorted_boxes, None, 0, &mut memo)
}

/// 再帰で最大高さを求める（メモ化付き）
fn max_height_recursive(
    boxes: &[BoxDim],
    bottom: Option<&BoxDim>,
    index: usize,
    memo: &mut [Option<u32>],
) -> u32 {
    if index >= boxes.len() {
        return 0;
    }

    let current = &boxes[index];

    let height_width = if bottom.is_none() || current.can_be_above(bottom.unwrap()) {
        if let Some(cached) = memo[index] {
            cached
        } else {
            let h = current.height + max_height_recursive(boxes, Some(current), index + 1, memo);
            memo[index] = Some(h);
            h
        }
    } else {
        0
    };
    let height_without = max_height_recursive(boxes, bottom, index + 1, memo);
    height_width.max(height_without)
}
