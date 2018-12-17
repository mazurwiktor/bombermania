trait Entity {
    fn new(id: u32) -> Self;
}

struct CommonEntity { id: u32 }

impl Entity for CommonEntity {
    fn new(id: u32) -> CommonEntity {
        CommonEntity { id: id }
    }
}
