pub struct Genre {
    pub attr_match: u8,
    pub value: String,
}

pub struct TitleInfo {
    
}

pub struct SrcTitleInfo {
    
}

pub struct DocumentInfo {
    
}

pub struct PublishInfo {
    
}

pub struct Description {
    pub title_info: TitleInfo,
    pub src_title_info: Vec<SrcTitleInfo>,
    pub document_info: DocumentInfo,
    pub publish_info: Vec<PublishInfo>,
}