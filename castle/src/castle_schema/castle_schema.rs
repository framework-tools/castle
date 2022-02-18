

pub const SCHEMA: &str = "
fn basic_page_info() -> BasicPageInfo
fn basic_parent_page_info() -> BasicPageInfo

type BasicPageInfo {
    title: String,
    icon: Option<String>,
    emoji: Option<String>
}

fn page_info() -> PageInfo

type PageInfo {
    id: uuid,
    basic_page_info: BasicPageInfo,
    description: String,
    parent_id: uuid,
    basic_parent_page_info: BasicPageInfo,
    blocks: Vec<Block>
}

enum Block {
    ContentBlock(ContentBlock),
    KanbanBlock(KanbanBlock),
    CheckListBlock(CheckListBlock),
}

type ContentBlock {
    id: uuid
}

type KanbanBlock {
    id: uuid
}

type CheckListBlock {
    id: uuid
}
";